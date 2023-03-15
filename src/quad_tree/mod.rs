use bevy::utils::HashSet;

use crate::{
    collision::CollisionPair,
    shape::AABB,
    world::broad::{BroadPhase, BroadPhaseElement},
};

const MAX_DEPTH: u8 = 8;
const MAX_CHILDREN: usize = 16;

enum QuadCorner {
    TopLeft = 0,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug)]
pub struct QuadTree<Handle>
where
    Handle: Clone, {
    bounds:   AABB<i32>,
    level:    u8,
    children: Vec<BroadPhaseElement<Handle>>,
    nodes:    Option<[Box<QuadTree<Handle>>; 4]>,
}

impl<Handle> QuadTree<Handle>
where
    Handle: Clone + PartialEq + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + Copy,
{
    pub fn new(level: u8, bounds: AABB<i32>) -> Self {
        QuadTree {
            bounds,
            level,
            children: Vec::with_capacity(MAX_CHILDREN),
            nodes: None,
        }
    }

    /**
     * Splits a node into 4 subnodes
     */
    fn split(&mut self) {
        if self.nodes.is_some() {
            unreachable!();
        }
        let half_width = self.bounds.width() / 2;
        let half_height = self.bounds.height() / 2;

        let x = self.bounds.min.x;
        let y = self.bounds.min.y;

        self.nodes = Some([
            Box::new(QuadTree::new(
                self.level + 1,
                AABB::new(x, y, half_width, half_height),
            )),
            Box::new(QuadTree::new(
                self.level + 1,
                AABB::new(x + half_width, y, half_width, half_height),
            )),
            Box::new(QuadTree::new(
                self.level + 1,
                AABB::new(x, y + half_height, half_width, half_height),
            )),
            Box::new(QuadTree::new(
                self.level + 1,
                AABB::new(x + half_width, y + half_height, half_width, half_height),
            )),
        ]);
    }

    fn inner_insert(&mut self, element: BroadPhaseElement<Handle>) {
        if !element.aabb.intersects(&self.bounds) {
            return;
        }
        if let Some(nodes) = &mut self.nodes {
            for node in nodes {
                node.inner_insert(element);
            }
        } else {
            self.children.push(element);
            if self.children.len() > MAX_CHILDREN && self.level < MAX_DEPTH {
                self.split();
                let children = std::mem::take(&mut self.children);
                for child in children {
                    for node in self.nodes.as_mut().unwrap() {
                        node.inner_insert(child);
                    }
                }
            }
        }
    }

    fn inner_check_collisions(&self, collision_set: &mut HashSet<CollisionPair<Handle>>) {
        if let Some(nodes) = &self.nodes {
            for node in nodes.iter() {
                node.inner_check_collisions(collision_set);
            }
        } else {
            for (i, a) in self.children.iter().enumerate() {
                // check for collisions with children within the same area
                for b in &self.children[(i + 1)..] {
                    if a.aabb.intersects(&b.aabb) {
                        collision_set.insert(CollisionPair {
                            a: a.handle,
                            b: b.handle,
                        });
                    }
                }
            }
        }
    }
}

impl<Handle: Clone + Copy + PartialEq + std::fmt::Debug + PartialEq + Eq + std::hash::Hash>
    BroadPhase<Handle> for QuadTree<Handle>
{
    fn insert(&mut self, element: BroadPhaseElement<Handle>) {
        self.inner_insert(element);
    }

    fn len(&self) -> usize {
        self.children.len()
            + self.nodes.as_ref().map_or(0, |nodes| {
                nodes.iter().fold(0, |sum, node| sum + node.len())
            })
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn remove(&mut self, element: BroadPhaseElement<Handle>) {
        if !element.aabb.intersects(&self.bounds) {
            return;
        }

        if let Some(nodes) = self.nodes.as_mut() {
            for node in nodes {
                node.remove(element);
            }
        } else if let Some(index) = self
            .children
            .iter()
            .position(|child| element.handle == child.handle)
        {
            self.children.swap_remove(index);
        }
    }

    fn check(&self, aabb: AABB<i32>, collisions: &mut HashSet<Handle>) {
        if !aabb.intersects(&self.bounds) {
            return;
        }
        if let Some(nodes) = &self.nodes {
            for node in nodes {
                node.check(aabb, collisions);
            }
            return;
        }
        for child in &self.children {
            if aabb.intersects(&child.aabb) {
                collisions.insert(child.handle);
            }
        }
    }

    fn check_collisions(&self) -> Vec<CollisionPair<Handle>> {
        let mut collisions = HashSet::new();

        self.inner_check_collisions(&mut collisions);

        collisions.into_iter().collect()
    }

    fn clear(&mut self) {
        self.children.clear();
        self.nodes = None;
    }

    fn clean_up(&mut self) -> bool {
        let children_is_empty = self.children.is_empty();

        let mut all_are_empty = true;
        if let Some(nodes) = self.nodes.as_mut() {
            for node in nodes.iter_mut() {
                if !node.clean_up() {
                    all_are_empty = false;
                }
            }
        }

        if all_are_empty && self.nodes.is_some() {
            self.nodes = None;
        }

        children_is_empty && all_are_empty
    }
}

#[cfg(test)]
mod tests {
    use generational_arena::Arena;

    use super::*;
    use crate::{
        body::Body,
        shape::{Circle, Shape},
        Vec2,
    };

    #[test]
    fn it_inserts_maximum_children() {
        let mut quad_tree = QuadTree::new(0, AABB::new(-1, -1, 2, 2));
        let mut bodies = Arena::new();

        for _ in 0..MAX_CHILDREN {
            let body = Body::default();

            quad_tree.insert(BroadPhaseElement {
                aabb:   body.get_aabb(),
                handle: bodies.insert(body),
            });
        }

        assert_eq!(quad_tree.children.len(), MAX_CHILDREN);
    }

    #[test]
    fn it_splits_into_quadrants() {
        let mut quad_tree = QuadTree::new(0, AABB::new(-10, -10, 20, 20));
        let mut bodies = Arena::new();

        let length = MAX_CHILDREN * 4;

        for i in 0..length {
            let x = ((i / (length / 4)) % 2) as f32 * 20.0 - 10.0;
            let y = (i / (length / 2)) as f32 * 20.0 - 10.0;

            let body = Body::new(
                1.0,
                1.0,
                Shape::Circle(Circle::new(0.1)),
                Vec2::new(x, y),
                false,
                false,
                Body::default().entity,
            );

            quad_tree.insert(BroadPhaseElement {
                aabb:   body.get_aabb(),
                handle: bodies.insert(body),
            });
        }

        assert_eq!(quad_tree.children.len(), 0);
        let nodes = quad_tree.nodes.as_ref().unwrap();

        assert_eq!(
            (
                nodes[QuadCorner::TopLeft as usize].children.len(),
                nodes[QuadCorner::TopRight as usize].children.len(),
                nodes[QuadCorner::BottomLeft as usize].children.len(),
                nodes[QuadCorner::BottomRight as usize].children.len()
            ),
            (MAX_CHILDREN, MAX_CHILDREN, MAX_CHILDREN, MAX_CHILDREN)
        );
    }

    #[test]
    fn it_removes_body() {
        let mut quad_tree = QuadTree::new(0, AABB::new(-10, -10, 20, 20));
        let mut bodies = Arena::new();
        let mut bodies_to_remove = vec![];

        let length = MAX_CHILDREN * 4;
        for i in 0..length {
            let x = ((i / (length / 4)) % 2) as f32 * 20.0 - 10.0;
            let y = (i / (length / 2)) as f32 * 20.0 - 10.0;

            let body = Body::new(
                1.0,
                1.0,
                Shape::Circle(Circle::new(0.1)),
                Vec2::new(x, y),
                false,
                false,
                Body::default().entity,
            );
            let aabb = body.get_aabb();
            let handle = bodies.insert(body);
            if x == -10.0 && y == -10.0 {
                bodies_to_remove.push(handle);
            }
            quad_tree.insert(BroadPhaseElement { handle, aabb });
        }
        bodies_to_remove.iter().for_each(|handle| {
            quad_tree.remove(BroadPhaseElement {
                handle: *handle,
                aabb:   bodies.get(*handle).unwrap().get_aabb(),
            });
        });
        assert_eq!(quad_tree.children.len(), 0);
        let nodes = quad_tree.nodes.as_ref().unwrap();

        assert_eq!(
            (
                nodes[QuadCorner::TopLeft as usize].children.len(),
                nodes[QuadCorner::TopRight as usize].children.len(),
                nodes[QuadCorner::BottomLeft as usize].children.len(),
                nodes[QuadCorner::BottomRight as usize].children.len()
            ),
            (0, MAX_CHILDREN, MAX_CHILDREN, MAX_CHILDREN)
        );
    }
}
