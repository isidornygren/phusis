use crate::body::Body;
use crate::checks::check_collision;
use crate::collision::Collision;
use crate::shape::AABB;

use std::sync::{Arc, Mutex};

const MAX_DEPTH: u8 = 8;
const MAX_CHILDREN: usize = 16;

enum QuadCorner {
    TopLeft = 0,
    TopRight,
    BottomLeft,
    BottomRight,
}

pub type WrappedBody = Arc<Mutex<Body>>;

/**
 * TODO: AABB should be integer based here
 */

#[derive(Debug)]
pub struct QuadTree {
    bounds: AABB,
    level: u8,
    children: Vec<WrappedBody>,
    nodes: Option<[Box<QuadTree>; 4]>,
}

impl QuadTree {
    pub fn new(level: u8, bounds: AABB) -> Self {
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
    pub fn insert(&mut self, body: WrappedBody) {
        let index = self.get_index(&body.lock().unwrap().get_aabb());
        if let (Some(nodes), Some(i)) = (&mut self.nodes, index) {
            nodes[i].insert(body);
            return;
        }
        self.children.push(body);
        if self.children.len() > MAX_CHILDREN && self.level < MAX_DEPTH {
            if self.nodes.is_none() {
                self.split();
            }
            let mut i = 0;
            while i < self.children.len() {
                let aabb = self.children[i].lock().unwrap().get_aabb();
                match self.get_index(&aabb) {
                    Some(quadrant_index) => {
                        self.nodes.as_mut().unwrap()[quadrant_index]
                            .insert(self.children.remove(i));
                    }
                    None => {
                        i += 1;
                    }
                }
            }
        }
    }
    /**
     * Removes an element from the quad tree
     */
    pub fn remove(&mut self, body: &WrappedBody) {
        if let Some(index) = self
            .children
            .iter()
            .position(|child| Arc::ptr_eq(body, child))
        {
            // Remove that index
            self.children.remove(index);
        } else {
            // Traverse to a child quad tree
            let unlocked_body = body.lock().unwrap();
            if let Some(quadrant_index) = self.get_index(&unlocked_body.get_aabb()) {
                drop(unlocked_body);
                if let Some(node) = self.nodes.as_mut() {
                    node[quadrant_index].remove(body);
                } else {
                    println!("Quadrant {} not found on quad tree", quadrant_index);
                    unreachable!();
                }
            } else {
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
        let half_width = self.bounds.get_width() / 2f32;
        let half_height = self.bounds.get_height() / 2f32;

        let x = self.bounds.get_x();
        let y = self.bounds.get_y();

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
    fn get_index(&self, bounds: &AABB) -> Option<usize> {
        let vertical_midpoint = self.bounds.get_vertical_mid();
        let horizontal_midpoint = self.bounds.get_horizontal_mid();

        // if object can completely fit within the top quadrants
        let top_quad = bounds.get_y() < vertical_midpoint && bounds.max.y < vertical_midpoint;
        let bottom_quad = bounds.get_y() > vertical_midpoint;

        if bounds.get_x() < horizontal_midpoint && bounds.max.x < horizontal_midpoint {
            if top_quad {
                return Some(QuadCorner::TopLeft as usize);
            } else if bottom_quad {
                return Some(QuadCorner::BottomLeft as usize);
            }
        } else if bounds.get_x() > horizontal_midpoint {
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
    pub fn retrieve(&self, item: AABB) -> Vec<WrappedBody> {
        let _index = self.get_index(&item);
        if let (Some(i), Some(nodes)) = (self.get_index(&item), &self.nodes) {
            nodes[i].retrieve(item)
        } else {
            self.get_children()
        }
    }

    pub fn get_children(&self) -> Vec<WrappedBody> {
        let mut nodes_children = self.get_node_children();
        nodes_children.extend(self.children.clone());
        nodes_children
    }

    pub fn get_node_children(&self) -> Vec<WrappedBody> {
        let mut nodes_children = vec![];
        if let Some(nodes) = &self.nodes {
            for node in nodes.iter() {
                nodes_children.extend(node.get_children());
            }
        }
        nodes_children
    }

    pub fn check_collisions(&self) -> Vec<Collision> {
        let mut collisions: Vec<Collision> = vec![];
        // first check for collision if there is a node child
        // with ALL the children
        let sub_children = self.get_node_children();
        // check for collision within its children and the
        // sub children
        for (i, a) in self.children.iter().enumerate() {
            // check for collisions with children within the same area
            for b in &self.children[(i + 1)..] {
                if let Some(collision) = check_collision(a, b) {
                    collisions.push(collision);
                }
            }
            // check for collisions with sub children
            for sub_child in &sub_children {
                if let Some(collision) = check_collision(a, sub_child) {
                    collisions.push(collision);
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

    pub fn get_node_aabb(&self) -> Vec<AABB> {
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
    use std::default;

    use crate::{
        shape::{Circle, Shape},
        Vec2,
    };

    use super::*;

    #[test]
    fn it_inserts_maximum_children() {
        let mut quad_tree = QuadTree::new(0, AABB::new(-1f32, -1f32, 1f32, 1f32));

        for _ in 0..MAX_CHILDREN {
            let body = Arc::new(Mutex::new(Body::default()));
            quad_tree.insert(body);
        }

        assert_eq!(quad_tree.children.len(), MAX_CHILDREN)
    }

    #[test]
    fn is_splits_into_quadrants() {
        let mut quad_tree = QuadTree::new(0, AABB::new(-10f32, -10f32, 10f32, 10f32));

        let length = MAX_CHILDREN * 4;
        for i in 0..length {
            let x = ((i / (length / 4)) % 2) as f32 * 20.0 - 10.0;
            let y = (i / (length / 2)) as f32 * 20.0 - 10.0;

            let body = Arc::new(Mutex::new(Body::new(
                1.0,
                1.0,
                Shape::Circle(Circle::new(0.1)),
                Vec2::new(x, y),
                false,
                false,
                Body::default().entity,
            )));
            quad_tree.insert(body);
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
        )
    }

    #[test]
    fn it_removes_body() {
        let mut quad_tree = QuadTree::new(0, AABB::new(-10f32, -10f32, 10f32, 10f32));

        let mut bodies = vec![];

        let length = MAX_CHILDREN * 4;
        for i in 0..length {
            let x = ((i / (length / 4)) % 2) as f32 * 20.0 - 10.0;
            let y = (i / (length / 2)) as f32 * 20.0 - 10.0;

            let body = Arc::new(Mutex::new(Body::new(
                1.0,
                1.0,
                Shape::Circle(Circle::new(0.1)),
                Vec2::new(x, y),
                false,
                false,
                Body::default().entity,
            )));
            if x == -10.0 && y == -10.0 {
                bodies.push(Arc::clone(&body));
            }
            quad_tree.insert(body);
        }
        bodies.iter().for_each(|body| {
            quad_tree.remove(body);
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
        )
    }
}
