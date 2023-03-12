use crate::{
    collision::BroadCollision,
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
    Handle: Clone,
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

    /**
     * Determine which node the object belongs to
     */
    fn get_index(&self, aabb: &AABB<i32>) -> Option<usize> {
        let vertical_midpoint = self.bounds.min.y + (self.bounds.height() / 2);
        let horizontal_midpoint = self.bounds.min.x + (self.bounds.width() / 2);

        // if object can completely fit within the top quadrants
        let top_quad = aabb.min.y < vertical_midpoint && aabb.max.y < vertical_midpoint;
        let bottom_quad = aabb.min.y > vertical_midpoint;

        if aabb.min.x < horizontal_midpoint && aabb.max.x < horizontal_midpoint {
            if top_quad {
                return Some(QuadCorner::TopLeft as usize);
            } else if bottom_quad {
                return Some(QuadCorner::BottomLeft as usize);
            }
        } else if aabb.min.x > horizontal_midpoint {
            if top_quad {
                return Some(QuadCorner::TopRight as usize);
            } else if bottom_quad {
                return Some(QuadCorner::BottomRight as usize);
            }
        }
        None
    }

    fn get_children(&self) -> Vec<BroadPhaseElement<Handle>> {
        let mut nodes_children = self.get_node_children();
        nodes_children.extend(self.children.clone());
        nodes_children
    }

    fn get_node_children(&self) -> Vec<BroadPhaseElement<Handle>> {
        let mut nodes_children = vec![];
        if let Some(nodes) = &self.nodes {
            for node in nodes.iter() {
                nodes_children.extend(node.get_children());
            }
        }
        nodes_children
    }
}

impl<Handle: Clone + PartialEq + std::fmt::Debug> BroadPhase<Handle> for QuadTree<Handle> {
    fn insert(&mut self, element: BroadPhaseElement<Handle>) {
        let index = self.get_index(&element.aabb);
        if let (Some(nodes), Some(i)) = (&mut self.nodes, index) {
            nodes[i].insert(element);
            return;
        }
        self.children.push(element);
        if self.children.len() > MAX_CHILDREN && self.level < MAX_DEPTH {
            if self.nodes.is_none() {
                self.split();
            }
            let mut i = 0;
            while i < self.children.len() {
                let element = &self.children[i];
                match self.get_index(&element.aabb) {
                    Some(quadrant_index) => {
                        self.nodes.as_mut().unwrap()[quadrant_index]
                            .insert(self.children.remove(i));
                    },
                    None => {
                        i += 1;
                    },
                }
            }
        }
    }

    fn remove(&mut self, element: BroadPhaseElement<Handle>) {
        if let Some(index) = self
            .children
            .iter()
            .position(|child| element.handle == child.handle)
        {
            // Remove that index
            self.children.remove(index);
        } else {
            // Traverse to a child quad tree
            if let Some(quadrant_index) = self.get_index(&element.aabb) {
                if let Some(node) = self.nodes.as_mut() {
                    node[quadrant_index].remove(element);
                } else {
                    unreachable!(
                        "Quadrant {} not found on quad tree\nElement: {:?}",
                        quadrant_index, element
                    );
                }
            }
        }
    }

    fn check(&self, aabb: AABB<i32>) -> Vec<Handle> {
        let index = self.get_index(&aabb);
        if let (Some(nodes), Some(i)) = (&self.nodes, index) {
            return nodes[i].check(aabb);
        }

        let mut collisions = vec![];

        // Either the element should only check in this node, or the node has no children
        for child in self.get_children() {
            if aabb.intersects(&child.aabb) {
                collisions.push(child.handle.clone());
            }
        }
        collisions
    }

    fn check_collisions(&self) -> Vec<BroadCollision<Handle>> {
        let mut collisions = vec![];
        // first check for collision if there is a node child
        // with ALL the children
        let sub_children = self.get_node_children();
        // check for collision within its children and the
        // sub children
        for (i, a) in self.children.iter().enumerate() {
            // check for collisions with children within the same area
            for b in &self.children[(i + 1)..] {
                if a.aabb.intersects(&b.aabb) {
                    collisions.push(BroadCollision {
                        a: a.handle.clone(),
                        b: b.handle.clone(),
                    });
                }
            }
            // check for collisions with sub children
            for sub_child in &sub_children {
                if a.aabb.intersects(&sub_child.aabb) {
                    collisions.push(BroadCollision {
                        a: a.handle.clone(),
                        b: sub_child.handle.clone(),
                    });
                }
            }
        }
        // Go deeper!
        if let Some(nodes) = &self.nodes {
            for node in nodes.iter() {
                collisions.extend(node.check_collisions());
            }
        }
        collisions
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
        let mut quad_tree = QuadTree::new(0, AABB::new(-1, -1, 1, 1));
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
    fn is_splits_into_quadrants() {
        let mut quad_tree = QuadTree::new(0, AABB::new(-10, -10, 10, 10));
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
        assert_eq!(
            (
                quad_tree.nodes.as_ref().unwrap()[QuadCorner::TopLeft as usize]
                    .children
                    .len(),
                quad_tree.nodes.as_ref().unwrap()[QuadCorner::TopRight as usize]
                    .children
                    .len(),
                quad_tree.nodes.as_ref().unwrap()[QuadCorner::BottomLeft as usize]
                    .children
                    .len(),
                quad_tree.nodes.as_ref().unwrap()[QuadCorner::BottomRight as usize]
                    .children
                    .len()
            ),
            (MAX_CHILDREN, MAX_CHILDREN, MAX_CHILDREN, MAX_CHILDREN)
        );
    }

    #[test]
    fn it_removes_body() {
        let mut quad_tree = QuadTree::new(0, AABB::new(-10, -10, 10, 10));
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
        assert_eq!(
            (
                quad_tree.nodes.as_ref().unwrap()[QuadCorner::TopLeft as usize]
                    .children
                    .len(),
                quad_tree.nodes.as_ref().unwrap()[QuadCorner::TopRight as usize]
                    .children
                    .len(),
                quad_tree.nodes.as_ref().unwrap()[QuadCorner::BottomLeft as usize]
                    .children
                    .len(),
                quad_tree.nodes.as_ref().unwrap()[QuadCorner::BottomRight as usize]
                    .children
                    .len()
            ),
            (0, MAX_CHILDREN, MAX_CHILDREN, MAX_CHILDREN)
        );
    }
}
