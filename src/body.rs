#[cfg(feature = "bevy")]
use bevy::prelude::Entity;

use crate::{
    shape::{Circle, Shape, AABB},
    Vec2,
};

#[derive(Debug)]
pub struct Body {
    pub position:    Vec2,
    pub velocity:    Vec2,
    pub force:       Vec2, // TODO: is this needed
    pub mass:        f32,
    pub inv_mass:    f32,
    pub restitution: f32,
    pub shape:       Shape,
    pub friction:    f32,
    pub fixed:       bool,
    pub sensor:      bool,
    #[cfg(feature = "bevy")]
    pub entity:      Entity,
}

impl Default for Body {
    fn default() -> Self {
        Self {
            position: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(0.0, 0.0),
            force: Vec2::new(0.0, 0.0),
            mass: 1.0,
            inv_mass: 1.0,
            restitution: 0.0,
            shape: Shape::Circle(Circle::new(1.0)),
            friction: 0.0,
            fixed: false,
            sensor: false,
            #[cfg(feature = "bevy")]
            entity: Entity::from_bits(0),
        }
    }
}

impl Body {
    #[must_use]
    pub fn new(
        mass: f32,
        restitution: f32,
        shape: Shape,
        position: Vec2,
        fixed: bool,
        sensor: bool,
        #[cfg(feature = "bevy")] entity: Entity,
    ) -> Self {
        Body {
            mass,
            restitution,
            inv_mass: 1f32 / mass,
            position,
            velocity: Vec2::new(0f32, 0f32),
            force: Vec2::new(0f32, 0f32),
            shape,
            friction: 5f32,
            fixed,
            sensor,
            #[cfg(feature = "bevy")]
            entity,
        }
    }

    #[must_use]
    pub fn get_aabb(&self) -> AABB {
        self.shape.get_aabb() + &self.position
    }
}
