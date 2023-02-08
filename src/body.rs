use crate::checks::circle_vs_circle;
use crate::collision::Collision;
use crate::shape::{Circle, Shape, ShapeKind, AABB};

use crate::Vec2;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Body {
    pub position: Vec2,
    pub velocity: Vec2,
    pub force: Vec2, // TODO: is this needed
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
        position: Vec2,
        fixed: bool,
    ) -> Self {
        return Body {
            mass,
            restitution,
            inv_mass: 1f32 / mass,
            position,
            velocity: Vec2::new(0f32, 0f32),
            force: Vec2::new(0f32, 0f32),
            shape,
            friction: 5f32,
            fixed,
        };
    }

    pub fn get_aabb(&self) -> AABB {
        self.shape.get_aabb() + &self.position
    }

    /*pub fn apply_force(&mut self, force: Vec2) {
        // self.force += force;
        self.force += force;
        // self.velocity = force * self.inv_mass * dt;
    }*/
}
