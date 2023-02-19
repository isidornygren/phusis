use bevy::prelude::*;

use crate::{
    shape::Shape,
    world::{BodyHandle, PhysicsWorld},
};

#[derive(Resource)]
pub struct PhysicsWorldResource {
    pub physics_world: PhysicsWorld,
}

#[derive(Component)]
pub struct Collider {
    pub shape: Shape,
    pub mass: f32,
    pub constitution: f32,
    pub fixed: bool,
}

#[derive(Component, Default)]
pub struct Collisions {
    pub entities: Vec<Entity>,
}

#[derive(Component)]
pub struct Sensor;

#[derive(Component)]
pub struct ComponentBodyHandle {
    pub handle: BodyHandle,
}

#[derive(Component)]
pub struct KinematicController {
    pub translation: Vec2,
}
