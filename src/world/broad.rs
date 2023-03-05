use super::BodyHandle;
use crate::{collision::BroadCollision, shape::AABB};

#[derive(Debug, Clone)]
pub struct BroadPhaseElement {
    pub aabb:   AABB<i32>,
    pub handle: BodyHandle,
}

impl BroadPhaseElement {
    #[must_use]
    pub fn collides_with(&self, other: &Self) -> bool {
        let pos_diff = (other.aabb.min - self.aabb.min).abs();

        let b_center = (other.aabb.max - other.aabb.min) / 2;
        let a_center = (self.aabb.max - self.aabb.min) / 2;

        let penetration = b_center + a_center - pos_diff;

        penetration.x > 0 && penetration.y > 0
    }
}

pub trait BroadPhase {
    fn insert(&mut self, element: BroadPhaseElement);
    fn remove(&mut self, element: BroadPhaseElement);
    fn check(&self, element: BroadPhaseElement) -> Vec<BroadCollision>;
    fn check_collisions(&self) -> Vec<BroadCollision>;
    fn clear(&mut self);
    /**
     * Removes any dangling nodes,
     * returns true if the node was empty
     */
    fn clean_up(&mut self) -> bool;
}
