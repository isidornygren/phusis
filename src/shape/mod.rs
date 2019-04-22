use nalgebra::Vector2;

pub mod aabb;
mod circle;

pub use aabb::AABB;
pub use circle::Circle;

fn distance_squared(vec: &Vector2<f32>) -> f32 {
    (vec.x).powf(2f32) + (vec.y).powf(2f32)
}

#[derive(Debug, Copy, Clone)]
pub enum ShapeKind {
    Circle,
    AABB,
}

pub trait Shape {
    fn get_kind(&self) -> ShapeKind;
    /*fn vs_circle(&self, other: Rc<RefCell<Body>>) -> Option<Collision>;
    fn vs_aabb(&self) -> Option<Collision>;*/
    fn get_radius(&self) -> f32;
    fn get_aabb(&self) -> AABB;
}
