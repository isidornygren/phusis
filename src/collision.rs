use crate::Vec2;

#[derive(Debug)]
pub struct Contact<T> {
    pub penetration_depth: T,
    pub normal:            Vec2<T>,
}

#[derive(Debug)]
pub struct Collision<T, Handle> {
    pub contact: Contact<T>,
    pub a:       Handle,
    pub b:       Handle,
}

#[derive(Debug)]
pub struct BroadCollision<Handle> {
    pub a: Handle,
    pub b: Handle,
}
