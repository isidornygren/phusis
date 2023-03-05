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
pub struct QuadTree {
    bounds:   AABB<i32>,
    level:    u8,
    children: Vec<BroadPhaseElement>,
    nodes:    Option<[Box<QuadTree>; 4]>,
}

impl QuadTree {
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
    fn get_index(&self, element: &BroadPhaseElement) -> Option<usize> {
        let vertical_midpoint = self.bounds.min.y + (self.bounds.height() / 2);
        let horizontal_midpoint = self.bounds.min.x + (self.bounds.width() / 2);

        // if object can completely fit within the top quadrants
        let top_quad =
            element.aabb.min.y < vertical_midpoint && element.aabb.max.y < vertical_midpoint;
        let bottom_quad = element.aabb.min.y > vertical_midpoint;

        if element.aabb.min.x < horizontal_midpoint && element.aabb.max.x < horizontal_midpoint {
            if top_quad {
                return Some(QuadCorner::TopLeft as usize);
            } else if bottom_quad {
                return Some(QuadCorner::BottomLeft as usize);
            }
        } else if element.aabb.min.x > horizontal_midpoint {
            if top_quad {
                return Some(QuadCorner::TopRight as usize);
            } else if bottom_quad {
                return Some(QuadCorner::BottomRight as usize);
            }
        }
        None
    }

    fn get_children(&self) -> Vec<BroadPhaseElement> {
        let mut nodes_children = self.get_node_children();
        nodes_children.extend(self.children.clone());
        nodes_children
    }

    fn get_node_children(&self) -> Vec<BroadPhaseElement> {
        let mut nodes_children = vec![];
        if let Some(nodes) = &self.nodes {
            for node in nodes.iter() {
                nodes_children.extend(node.get_children());
            }
        }
        nodes_children
    }
}

impl BroadPhase for QuadTree {
    fn insert(&mut self, element: BroadPhaseElement) {
        let index = self.get_index(&element);
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
                match self.get_index(element) {
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

    fn remove(&mut self, element: BroadPhaseElement) {
        if let Some(index) = self
            .children
            .iter()
            .position(|child| element.handle.index == child.handle.index)
        {
            // Remove that index
            self.children.remove(index);
        } else {
            // Traverse to a child quad tree
            if let Some(quadrant_index) = self.get_index(&element) {
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

    fn check(&self, element: BroadPhaseElement) -> Vec<BroadCollision> {
        let index = self.get_index(&element);
        if let (Some(nodes), Some(i)) = (&self.nodes, index) {
            return nodes[i].check(element);
        }

        let mut collisions = vec![];

        // Either the element should only check in this node, or the node has no children
        for child in self.get_children() {
            if element.collides_with(&child) {
                collisions.push(BroadCollision {
                    a: element.handle.clone(),
                    b: child.handle.clone(),
                });
            }
        }
        collisions
    }

    fn check_collisions(&self) -> Vec<BroadCollision> {
        let mut collisions: Vec<BroadCollision> = vec![];
        // first check for collision if there is a node child
        // with ALL the children
        let sub_children = self.get_node_children();
        // check for collision within its children and the
        // sub children
        for (i, a) in self.children.iter().enumerate() {
            // check for collisions with children within the same area
            for b in &self.children[(i + 1)..] {
                if a.collides_with(b) {
                    collisions.push(BroadCollision {
                        a: a.handle.clone(),
                        b: b.handle.clone(),
                    });
                }
            }
            // check for collisions with sub children
            for sub_child in &sub_children {
                if a.collides_with(sub_child) {
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
    use super::*;
    use crate::{
        body::Body,
        shape::{Circle, Shape},
        world::BodyHandle,
        Vec2,
    };

    #[test]
    fn it_inserts_maximum_children() {
        let mut quad_tree = QuadTree::new(0, AABB::new(-1, -1, 1, 1));
        let mut bodies = vec![];

        for _ in 0..MAX_CHILDREN {
            let body = Body::default();
            let handle = BodyHandle {
                index: bodies.len(),
            };
            quad_tree.insert(BroadPhaseElement {
                handle,
                aabb: body.get_aabb(),
            });
            bodies.push(body);
        }

        assert_eq!(quad_tree.children.len(), MAX_CHILDREN);
    }

    #[test]
    fn is_splits_into_quadrants() {
        let mut quad_tree = QuadTree::new(0, AABB::new(-10, -10, 10, 10));
        let mut bodies = vec![];

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
            let handle = BodyHandle {
                index: bodies.len(),
            };
            quad_tree.insert(BroadPhaseElement {
                handle,
                aabb: body.get_aabb(),
            });
            bodies.push(body);
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
        let mut bodies = vec![];
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
            let handle = BodyHandle {
                index: bodies.len(),
            };
            if x == -10.0 && y == -10.0 {
                bodies_to_remove.push(handle.clone());
            }
            quad_tree.insert(BroadPhaseElement {
                handle,
                aabb: body.get_aabb(),
            });
            bodies.push(body);
        }
        bodies_to_remove.iter().for_each(|handle| {
            quad_tree.remove(BroadPhaseElement {
                handle: handle.clone(),
                aabb:   bodies.get(handle.index).unwrap().get_aabb(),
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
