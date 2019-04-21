use crate::collision::Collision;

use crate::body::Body;

use std::cell::RefCell;
use std::rc::Rc;

use nalgebra::Vector2;

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
}

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
    /*fn vs_circle(&self, other: Rc<RefCell<Body>>) -> Option<Collision> {
        let normal = other.position - self.position;

        let radius = (self.radius + other.shape.radius).powf(2f32);

        let distance_sqr = distance_squared(&normal);

        if distance_sqr > radius {
            return None;
        }
        let distance = distance_sqr.sqrt();

        if distance != 0f32 {
            return Some(Collision {
                penetration_depth: (a.shape.radius + b.shape.radius) - distance,
                normal: normal / distance,
            });
        } else {
            // Circles are on the same position
            // Choose random (but consistent) values
            return Some(Collision {
                penetration_depth: a.shape.radius,
                normal: Vector2::new(1f32, 0f32),
            });
        }
    }
    fn vs_aabb(&self) -> Option<Collision> {
        unimplemented!();
    }*/
}
