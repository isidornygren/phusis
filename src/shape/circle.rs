use crate::shape::AABB;

#[derive(Debug, Clone)]
pub struct Circle {
    pub radius: f32,
    aabb: AABB,
}

impl Circle {
    #[must_use]
    pub fn new(radius: f32) -> Self {
        // A circles aabb is centered around 0
        Circle {
            radius,
            aabb: AABB::new(-radius, -radius, radius * 2.0, radius * 2.0),
        }
    }
}

impl Circle {
    pub fn get_aabb(&self) -> AABB {
        self.aabb.clone()
    }
}
