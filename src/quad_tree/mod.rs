use crate::body::Body;
use crate::checks::check_collision;
use crate::collision::Collision;
use crate::shape::AABB;
use nalgebra::Vector2;

use std::cell::RefCell;
use std::rc::Rc;

const MAX_DEPTH: u8 = 5;
const MAX_CHILDREN: usize = 10;

/**
 * TODO: Set children to a fixed array as max children will be a small number
 * TODO: AABB should be integer based here
 */

pub struct QuadTree {
    bounds: AABB,
    // needed?
    level: u8,
    children: Vec<Rc<RefCell<Body>>>,
    nodes: Vec<QuadTree>,
}

impl QuadTree {
    pub fn new(level: u8, bounds: AABB) -> Self {
        return QuadTree {
            bounds,
            level,
            children: vec![],
            nodes: vec![],
        };
    }
    /**
     * Inserts an items into the quad tree
     */
    pub fn insert(&mut self, body: Rc<RefCell<Body>>) {
        if self.nodes.len() > 0 {
            let index = self.get_index(&body.borrow().get_aabb());
            if index.is_some() {
                self.nodes[index.unwrap()].insert(body);
                return;
            }
        }
        self.children.push(body);
        if self.children.len() > MAX_CHILDREN && self.level < MAX_DEPTH {
            if self.nodes.len() == 0 {
                self.split();
            }
            let mut i = 0;
            while i < self.children.len() {
                let aabb = self.children[i].borrow().get_aabb();
                match self.get_index(&aabb) {
                    Some(j) => {
                        self.nodes[j].insert(self.children.remove(i));
                    }
                    None => {
                        i += 1;
                    }
                }
            }
        }
    }
    /**
     * Clears all items from a quad tree
     */
    pub fn clear(&mut self) {
        self.children.clear();
        for node in self.nodes.iter_mut() {
            node.clear();
        }
        self.nodes.clear();
    }
    /**
     * Splits a node into 4 subnodes
     */
    pub fn split(&mut self) {
        assert!(self.nodes.len() == 0, "Node already split");
        let sub_width = self.bounds.get_width() / 2f32;
        let sub_height = self.bounds.get_height() / 2f32;

        let x = self.bounds.get_x();
        let y = self.bounds.get_y();

        self.nodes = vec![
            QuadTree::new(self.level + 1, AABB::new(x, y, sub_width, sub_height)),
            QuadTree::new(
                self.level + 1,
                AABB::new(x + sub_width, y, sub_width, sub_height),
            ),
            QuadTree::new(
                self.level + 1,
                AABB::new(x, y + sub_height, sub_width, sub_height),
            ),
            QuadTree::new(
                self.level + 1,
                AABB::new(x + sub_width, y + sub_height, sub_width, sub_height),
            ),
        ];
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
                // top left
                return Some(0);
            } else if bottom_quad {
                // bottom left
                return Some(2);
            }
        } else if bounds.get_x() > horizontal_midpoint {
            if top_quad {
                // top right
                return Some(1);
            } else if bottom_quad {
                // bottom right
                return Some(3);
            }
        }
        return None;
    }
    /**
     * Retrives all items in the same node as the specified item, if the specified item
     * overlaps the bounds of a node, then all nodes from the parent node will be retrieved
     */
    pub fn retrieve(&self, item: AABB) -> Vec<Rc<RefCell<Body>>> {
        let index = self.get_index(&item);
        if index.is_some() && self.nodes.len() > 0 {
            // let nodes = self.nodes.unwrap();
            return self.nodes[index.unwrap()].retrieve(item);
        } else {
            // should return all the nodes childrens as well
            return self.children.clone();
        }
    }

    pub fn get_children(&self) -> Vec<Rc<RefCell<Body>>> {
        let mut nodes_children = self.get_node_children();
        nodes_children.extend(self.children.clone());
        return nodes_children;
    }

    pub fn get_node_children(&self) -> Vec<Rc<RefCell<Body>>> {
        let mut nodes_children = vec![];
        for node in self.nodes.iter() {
            nodes_children.extend(node.get_children());
        }
        return nodes_children;
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
                match check_collision(&a, &b) {
                    Some(collision) => collisions.push(collision),
                    _ => {}
                }
            }
            // check for collisions with sub children
            for sub_child in sub_children.iter() {
                match check_collision(&a, &sub_child) {
                    Some(collision) => collisions.push(collision),
                    _ => {}
                }
            }
        }
        // Go deeper!
        for node in self.nodes.iter() {
            collisions.extend(node.check_collisions());
        }
        return collisions;
    }

    pub fn get_node_aabb(&self) -> Vec<AABB> {
        let mut aabb_vec = vec![self.bounds.clone()];
        if self.nodes.len() > 0 {
            for node in self.nodes.iter() {
                aabb_vec.extend(node.get_node_aabb());
            }
        }
        return aabb_vec;
    }
}
