use crate::{collision::CollisionPair, shape::AABB};

#[derive(Debug, Clone, Copy)]
pub struct BroadPhaseElement<Handle> {
    pub aabb:   AABB<i32>,
    pub handle: Handle,
}

// impl<Handle> BroadPhaseElement<Handle>
// where
//     Handle: Clone,
// {
//     #[must_use]
//     pub fn collides_with(&self, other: &Self) -> bool {
//         let pos_diff = (other.aabb.min - self.aabb.min).abs();

//         let b_center = (other.aabb.max - other.aabb.min) / 2;
//         let a_center = (self.aabb.max - self.aabb.min) / 2;

//         let penetration = b_center + a_center - pos_diff;

//         penetration.x > 0 && penetration.y > 0
//     }
// }

pub trait BroadPhase<Handle>
where
    Handle: Clone + Eq + PartialEq + std::hash::Hash, {
    fn insert(&mut self, element: BroadPhaseElement<Handle>);
    fn remove(&mut self, element: BroadPhaseElement<Handle>);
    fn check(&self, element: AABB<i32>) -> Vec<Handle>;
    fn check_collisions(&self) -> Vec<CollisionPair<Handle>>;
    fn clear(&mut self);
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    /**
     * Removes any dangling nodes,
     * returns true if the node was empty
     */
    fn clean_up(&mut self) -> bool;
}
