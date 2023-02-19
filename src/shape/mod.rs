pub mod aabb;
mod circle;

pub use aabb::AABB;
pub use circle::Circle;

#[derive(Debug, Clone)]
pub enum Shape {
    Circle(Circle),
    AABB(AABB),
}

impl Shape {
    #[must_use]
    pub fn get_aabb(&self) -> &AABB {
        match self {
            Shape::Circle(circle) => circle.get_aabb(),
            Shape::AABB(aabb) => aabb,
        }
    }
}
