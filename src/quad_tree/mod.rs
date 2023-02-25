use crate::{
    collision::{Collision, Contact},
    shape::AABB,
    world::BodyHandle,
    Vec2,
};

const MAX_DEPTH: u8 = 8;
const MAX_CHILDREN: usize = 16;

#[derive(Debug, Clone)]
pub struct QuadElement {
    pub aabb:   AABB<i32>,
    pub handle: BodyHandle,
}

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
    children: Vec<QuadElement>,
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
     * Inserts an items into the quad tree
     */
    pub fn insert(&mut self, element: QuadElement) {
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

    /**
     * Removes an element from the quad tree
     */
    pub fn remove(&mut self, element: QuadElement) {
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

    /**
     * Clears all items from a quad tree
     */
    pub fn clear(&mut self) {
        self.children.clear();
        self.nodes = None;
    }

    /**
     * Splits a node into 4 subnodes
     */
    pub fn split(&mut self) {
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
    fn get_index(&self, element: &QuadElement) -> Option<usize> {
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

    /**
     * Retrieves all items in the same node as the specified item, if the specified item
     * overlaps the bounds of a node, then all nodes from the parent node will be retrieved
     */
    // pub fn retrieve(&self, item: AABB) -> Vec<QuadElement> {
    //     let _index = self.get_index(&item);
    //     if let (Some(i), Some(nodes)) = (self.get_index(&item), &self.nodes) {
    //         nodes[i].retrieve(item)
    //     } else {
    //         self.get_children()
    //     }
    // }

    pub fn get_children(&self) -> Vec<QuadElement> {
        let mut nodes_children = self.get_node_children();
        nodes_children.extend(self.children.clone());
        nodes_children
    }

    pub fn get_node_children(&self) -> Vec<QuadElement> {
        let mut nodes_children = vec![];
        if let Some(nodes) = &self.nodes {
            for node in nodes.iter() {
                nodes_children.extend(node.get_children());
            }
        }
        nodes_children
    }

    fn check_collision(a: &QuadElement, b: &QuadElement) -> Option<Contact<i32>> {
        let pos_diff = (b.aabb.min - a.aabb.min).abs();

        let b_center = (b.aabb.max - b.aabb.min) / 2;
        let a_center = (a.aabb.max - a.aabb.min) / 2;

        let penetration = b_center + a_center - pos_diff;
        if penetration.x <= 0 || penetration.y <= 0 {
            return None;
        }
        if penetration.x < penetration.y {
            let sign_x = pos_diff.x.signum();
            return Some(Contact {
                penetration_depth: penetration.x * sign_x,
                normal:            Vec2::new(sign_x, 0),
            });
        }
        let sign_y = pos_diff.y.signum();
        Some(Contact {
            penetration_depth: penetration.y * sign_y,
            normal:            Vec2::new(0, sign_y),
        })
    }

    /**
     * Broad collision checking
     */
    pub fn check_collisions(&self) -> Vec<Collision<i32>> {
        let mut collisions: Vec<Collision<i32>> = vec![];
        // first check for collision if there is a node child
        // with ALL the children
        let sub_children = self.get_node_children();
        // check for collision within its children and the
        // sub children
        for (i, a) in self.children.iter().enumerate() {
            // check for collisions with children within the same area
            for b in &self.children[(i + 1)..] {
                if let Some(contact) = Self::check_collision(a, b) {
                    collisions.push(Collision {
                        contact,
                        a: a.handle.clone(),
                        b: b.handle.clone(),
                    })
                }
            }
            // check for collisions with sub children
            for sub_child in &sub_children {
                if let Some(contact) = Self::check_collision(a, sub_child) {
                    collisions.push(Collision {
                        contact,
                        a: a.handle.clone(),
                        b: sub_child.handle.clone(),
                    })
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

    pub fn get_node_aabb(&self) -> Vec<AABB<i32>> {
        let mut aabb_vec = vec![self.bounds.clone()];
        if let Some(nodes) = &self.nodes {
            for node in nodes.iter() {
                aabb_vec.extend(node.get_node_aabb());
            }
        }
        aabb_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        body::Body,
        shape::{Circle, Shape},
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
            quad_tree.insert(QuadElement {
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
            quad_tree.insert(QuadElement {
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
            quad_tree.insert(QuadElement {
                handle,
                aabb: body.get_aabb(),
            });
            bodies.push(body);
        }
        bodies_to_remove.iter().for_each(|handle| {
            quad_tree.remove(QuadElement {
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
