use crate::checks::circle_vs_circle;
use crate::collision::Collision;
use crate::shape::{Circle, Shape, ShapeKind, AABB};

use nalgebra::Vector2;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Body {
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub force: Vector2<f32>, // TODO: is this needed
    // pub acceleration: f32,
    pub mass: f32,
    pub inv_mass: f32, // 1 / mass
    pub restitution: f32,
    pub shape: Box<Shape>,
    pub friction: f32,
    pub fixed: bool,
}

impl Body {
    pub fn new(
        mass: f32,
        restitution: f32,
        shape: Box<Shape>,
        position: Vector2<f32>,
        fixed: bool,
    ) -> Self {
        return Body {
            mass,
            restitution,
            inv_mass: 1f32 / mass,
            position,
            velocity: Vector2::new(0f32, 0f32),
            force: Vector2::new(0f32, 0f32),
            shape,
            friction: 5f32,
            fixed,
        };
    }

    pub fn get_aabb(&self) -> AABB {
        self.shape.get_aabb() + self.position
    }

    /*pub fn apply_force(&mut self, force: Vector2<f32>) {
        // self.force += force;
        self.force += force;
        // self.velocity = force * self.inv_mass * dt;
    }*/
}
