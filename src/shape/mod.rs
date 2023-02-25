pub mod aabb;
mod circle;

pub use aabb::AABB;
pub use circle::Circle;

use crate::Vec2;

#[derive(Debug, Clone)]
pub enum Shape {
    Circle(Circle),
    Rect(Vec2<f32>),
}

impl Shape {
    #[must_use]
    pub fn get_aabb(&self, position: Vec2<f32>) -> AABB<i32> {
        match self {
            Shape::Circle(circle) => {
                let i_radius = circle.radius.ceil() as i32;

                AABB::new(
                    position.x.floor() as i32 - i_radius,
                    position.y.floor() as i32 - i_radius,
                    i_radius * 2,
                    i_radius * 2,
                )
            },
            Shape::Rect(rect) => AABB::new(
                position.x.floor() as i32,
                position.y.floor() as i32,
                rect.x.ceil() as i32,
                rect.y.ceil() as i32,
            ),
        }
    }
}
