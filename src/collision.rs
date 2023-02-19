use crate::{world::BodyHandle, Vec2};

pub struct Collision {
    pub penetration_depth: f32,
    pub normal:            Vec2,
    pub a:                 BodyHandle,
    pub b:                 BodyHandle,
}
