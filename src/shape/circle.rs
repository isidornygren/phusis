use crate::shape::{Shape, ShapeKind, AABB};
use nalgebra::Vector2;

#[derive(Debug, Copy, Clone)]
pub struct Circle {
    pub radius: f32,
}

impl Shape for Circle {
    fn get_kind(&self) -> ShapeKind {
        return ShapeKind::Circle;
    }
    fn get_radius(&self) -> f32 {
        return self.radius;
    }
    fn get_aabb(&self) -> AABB {
        // TODO: Maybe store this as an object in circle instead of always
        // constructing it here.
        return AABB {
            min: Vector2::new(-self.radius, -self.radius),
            max: Vector2::new(self.radius, self.radius),
        };
    }
}
