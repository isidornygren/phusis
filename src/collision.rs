use crate::Vec2;
use std::cell::RefCell;
use std::rc::Rc;

use crate::body::Body;

pub struct Collision {
    pub penetration_depth: f32,
    pub normal: Vec2,
    pub a: Rc<RefCell<Body>>,
    pub b: Rc<RefCell<Body>>,
}

pub struct Manifold {
    a: Body, // TODO: Put these in a refcounted object
    b: Body,
    collision: Collision,
}
