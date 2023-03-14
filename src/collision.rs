use crate::Vec2;

#[derive(Debug)]
pub struct Contact<T> {
    pub penetration_depth: T,
    pub normal:            Vec2<T>,
}

#[derive(Debug)]
pub struct Collision<T, Handle>
where
    Handle: Eq + std::hash::Hash + PartialEq, {
    pub contact: Contact<T>,
    pub pair:    CollisionPair<Handle>,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct CollisionPair<Handle>
where
    Handle: Eq + std::hash::Hash + PartialEq, {
    pub a: Handle,
    pub b: Handle,
}
