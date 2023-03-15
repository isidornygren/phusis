use bevy::utils::HashSet;

use crate::{
    collision::CollisionPair,
    shape::AABB,
    world::broad::{BroadPhase, BroadPhaseElement},
};

const MAX_DEPTH: u8 = 8;
const MAX_CHILDREN: usize = 16;

#[derive(Debug)]
pub enum Node<Handle>
where
    Handle: Clone, {
    Branch([Box<QuadTree<Handle>>; 4]),
    Leaf(Vec<BroadPhaseElement<Handle>>),
}

impl<Handle: Clone> Node<Handle> {
    fn is_branch(&self) -> bool {
        matches!(self, Node::Branch(_))
    }

    fn is_leaf(&self) -> bool {
        matches!(self, Node::Leaf(_))
    }
}

#[derive(Debug)]
pub struct QuadTree<Handle>
where
    Handle: Clone, {
    bounds: AABB<i32>,
    level:  u8,
    node:   Node<Handle>,
}

impl<Handle> QuadTree<Handle>
where
    Handle: Clone + PartialEq + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + Copy,
{
    #[must_use]
    pub fn new(level: u8, bounds: AABB<i32>) -> Self {
        QuadTree {
            bounds,
            level,
            node: Node::Leaf(Vec::with_capacity(MAX_CHILDREN)),
        }
    }

    /**
     * Splits a node into 4 subnodes
     */
    fn split(&mut self) {
        if self.node.is_branch() {
            unreachable!("Trying to split already split node");
        }
        let half_width = self.bounds.width() / 2;
        let half_height = self.bounds.height() / 2;

        let x = self.bounds.min.x;
        let y = self.bounds.min.y;

        let leaf = std::mem::replace(
            &mut self.node,
            Node::Branch([
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
            ]),
        );

        if let (Node::Leaf(children), Node::Branch(nodes)) = (leaf, &mut self.node) {
            for child in children {
                for node in nodes.as_mut() {
                    node.inner_insert(child);
                }
            }
        } else {
            unreachable!();
        }
    }

    fn inner_insert(&mut self, element: BroadPhaseElement<Handle>) {
        if !element.aabb.intersects(&self.bounds) {
            return;
        }
        match &mut self.node {
            Node::Branch(nodes) => {
                for node in nodes {
                    node.inner_insert(element);
                }
            },
            Node::Leaf(children) => {
                children.push(element);
                if children.len() > MAX_CHILDREN && self.level < MAX_DEPTH {
                    self.split();
                }
            },
        }
    }

    fn inner_check_collisions(&self, collision_set: &mut HashSet<CollisionPair<Handle>>) {
        match &self.node {
            Node::Branch(nodes) => {
                for node in nodes.iter() {
                    node.inner_check_collisions(collision_set);
                }
            },
            Node::Leaf(children) => {
                for (i, a) in children.iter().enumerate() {
                    // check for collisions with children within the same area
                    for b in &children[(i + 1)..] {
                        if a.aabb.intersects(&b.aabb) {
                            collision_set.insert(CollisionPair {
                                a: a.handle,
                                b: b.handle,
                            });
                        }
                    }
                }
            },
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
        match &self.node {
            Node::Leaf(children) => children.len(),
            Node::Branch(nodes) => nodes.iter().fold(0, |sum, node| sum + node.len()),
        }
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn remove(&mut self, element: BroadPhaseElement<Handle>) {
        if !element.aabb.intersects(&self.bounds) {
            return;
        }

        match &mut self.node {
            Node::Branch(nodes) => {
                for node in nodes {
                    node.remove(element);
                }
            },
            Node::Leaf(children) => {
                if let Some(index) = children
                    .iter()
                    .position(|child| element.handle == child.handle)
                {
                    children.swap_remove(index);
                }
            },
        };
    }

    fn check(&self, aabb: AABB<i32>, collisions: &mut HashSet<Handle>) {
        if !aabb.intersects(&self.bounds) {
            return;
        }
        match &self.node {
            Node::Branch(nodes) => {
                for node in nodes {
                    node.check(aabb, collisions);
                }
            },
            Node::Leaf(children) => {
                for child in children {
                    if aabb.intersects(&child.aabb) {
                        collisions.insert(child.handle);
                    }
                }
            },
        }
    }

    fn check_collisions(&self) -> Vec<CollisionPair<Handle>> {
        let mut collisions = HashSet::new();

        self.inner_check_collisions(&mut collisions);

        collisions.into_iter().collect()
    }

    fn clear(&mut self) {
        self.node = Node::Leaf(vec![]);
    }

    fn clean_up(&mut self) -> bool {
        let mut children_is_empty = false;
        let mut all_are_empty = true;
        match &mut self.node {
            Node::Branch(nodes) => {
                for node in nodes.iter_mut() {
                    if !node.clean_up() {
                        all_are_empty = false;
                    }
                }
            },
            Node::Leaf(children) => {
                children_is_empty = children.is_empty();
            },
        }

        if all_are_empty && self.node.is_branch() {
            self.clear();
        }

        children_is_empty && all_are_empty
    }
}

// #[cfg(test)]
// mod tests {
//     use generational_arena::Arena;

//     use super::*;
//     use crate::{
//         body::Body,
//         shape::{Circle, Shape},
//         Vec2,
//     };

//     #[test]
//     fn it_inserts_maximum_children() {
//         let mut quad_tree = QuadTree::new(0, AABB::new(-1, -1, 2, 2));
//         let mut bodies = Arena::new();

//         for _ in 0..MAX_CHILDREN {
//             let body = Body::default();

//             quad_tree.insert(BroadPhaseElement {
//                 aabb:   body.get_aabb(),
//                 handle: bodies.insert(body),
//             });
//         }

//         assert_eq!(quad_tree.children.len(), MAX_CHILDREN);
//     }

//     #[test]
//     fn it_splits_into_quadrants() {
//         let mut quad_tree = QuadTree::new(0, AABB::new(-10, -10, 20, 20));
//         let mut bodies = Arena::new();

//         let length = MAX_CHILDREN * 4;

//         for i in 0..length {
//             let x = ((i / (length / 4)) % 2) as f32 * 20.0 - 10.0;
//             let y = (i / (length / 2)) as f32 * 20.0 - 10.0;

//             let body = Body::new(
//                 1.0,
//                 1.0,
//                 Shape::Circle(Circle::new(0.1)),
//                 Vec2::new(x, y),
//                 false,
//                 false,
//                 Body::default().entity,
//             );

//             quad_tree.insert(BroadPhaseElement {
//                 aabb:   body.get_aabb(),
//                 handle: bodies.insert(body),
//             });
//         }

//         assert_eq!(quad_tree.children.len(), 0);
//         let nodes = quad_tree.nodes.as_ref().unwrap();

//         assert_eq!(
//             (
//                 nodes[0].children.len(),
//                 nodes[1].children.len(),
//                 nodes[2].children.len(),
//                 nodes[3].children.len()
//             ),
//             (MAX_CHILDREN, MAX_CHILDREN, MAX_CHILDREN, MAX_CHILDREN)
//         );
//     }

//     #[test]
//     fn it_removes_body() {
//         let mut quad_tree = QuadTree::new(0, AABB::new(-10, -10, 20, 20));
//         let mut bodies = Arena::new();
//         let mut bodies_to_remove = vec![];

//         let length = MAX_CHILDREN * 4;
//         for i in 0..length {
//             let x = ((i / (length / 4)) % 2) as f32 * 20.0 - 10.0;
//             let y = (i / (length / 2)) as f32 * 20.0 - 10.0;

//             let body = Body::new(
//                 1.0,
//                 1.0,
//                 Shape::Circle(Circle::new(0.1)),
//                 Vec2::new(x, y),
//                 false,
//                 false,
//                 Body::default().entity,
//             );
//             let aabb = body.get_aabb();
//             let handle = bodies.insert(body);
//             if x == -10.0 && y == -10.0 {
//                 bodies_to_remove.push(handle);
//             }
//             quad_tree.insert(BroadPhaseElement { handle, aabb });
//         }
//         bodies_to_remove.iter().for_each(|handle| {
//             quad_tree.remove(BroadPhaseElement {
//                 handle: *handle,
//                 aabb:   bodies.get(*handle).unwrap().get_aabb(),
//             });
//         });
//         assert_eq!(quad_tree.children.len(), 0);
//         let nodes = quad_tree.nodes.as_ref().unwrap();

//         assert_eq!(
//             (
//                 nodes[0].children.len(),
//                 nodes[1].children.len(),
//                 nodes[2].children.len(),
//                 nodes[3].children.len()
//             ),
//             (0, MAX_CHILDREN, MAX_CHILDREN, MAX_CHILDREN)
//         );
//     }
// }
