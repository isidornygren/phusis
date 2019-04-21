use nalgebra::Vector2;

use crate::body::Body;
use crate::shape::Shape;

pub struct Collision {
    pub penetration_depth: f32,
    pub normal: Vector2<f32>,
}

pub struct Manifold {
    a: Body, // TODO: Put these in a refcounted object
    b: Body,
    collision: Collision,
}
