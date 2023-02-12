use crate::Vec2;
pub mod aabb;
mod circle;

pub use aabb::AABB;
pub use circle::Circle;

fn distance_squared(vec: &Vec2) -> f32 {
    (vec.x).powf(2f32) + (vec.y).powf(2f32)
}

#[derive(Debug, Clone)]
pub enum Shape {
    Circle(Circle),
    AABB(AABB),
}

impl Shape {
    pub fn get_aabb(&self) -> &AABB {
        match self {
            Shape::Circle(circle) => circle.get_aabb(),
            Shape::AABB(aabb) => aabb,
        }
    }
}
