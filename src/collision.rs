use crate::{world::BodyHandle, Vec2};

#[derive(Debug)]
pub struct Contact<T> {
    pub penetration_depth: T,
    pub normal:            Vec2<T>,
}

#[derive(Debug)]
pub struct Collision<T> {
    pub contact: Contact<T>,
    pub a:       BodyHandle,
    pub b:       BodyHandle,
}

#[derive(Debug)]
pub struct BroadCollision {
    pub a: BodyHandle,
    pub b: BodyHandle,
}
