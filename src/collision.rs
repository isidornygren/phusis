use crate::Vec2;

#[derive(Debug)]
pub struct Contact<T> {
    pub penetration_depth: T,
    pub normal:            Vec2<T>,
}

#[derive(Debug)]
pub struct Collision<T, Handle> {
    pub contact: Contact<T>,
    pub pair:    CollisionPair<Handle>,
}

#[derive(Debug, Clone, Copy)]
pub struct CollisionPair<Handle> {
    pub a: Handle,
    pub b: Handle,
}
