use crate::quad_tree::WrappedBody;
use crate::world::BodyHandle;
use crate::Vec2;

use crate::body::Body;

pub struct Collision {
    pub penetration_depth: f32,
    pub normal: Vec2,
    pub a: WrappedBody,
    pub b: WrappedBody,
}

pub struct Manifold {
    a: Body, // TODO: Put these in a refcounted object
    b: Body,
    collision: Collision,
}
