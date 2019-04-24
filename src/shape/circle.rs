use crate::shape::{Shape, ShapeKind, AABB};
use nalgebra::Vector2;

#[derive(Debug, Copy, Clone)]
pub struct Circle {
    pub radius: f32,
    aabb: AABB,
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        // A circles aabb is centered around 0
        return Circle {
            radius,
            aabb: AABB {
                min: Vector2::new(-radius, -radius),
                max: Vector2::new(radius, radius),
            },
        };
    }
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
        return self.aabb;
    }
}
