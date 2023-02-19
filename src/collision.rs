use crate::world::BodyHandle;
use crate::Vec2;

use crate::body::Body;

pub struct Collision {
    pub penetration_depth: f32,
    pub normal: Vec2,
    pub a: BodyHandle,
    pub b: BodyHandle,
}

pub struct Manifold {
    a: Body, // TODO: Put these in a refcounted object
    b: Body,
    collision: Collision,
}
