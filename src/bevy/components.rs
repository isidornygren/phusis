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
pub struct ComponentBody {
    pub shape: Shape,
    pub mass: f32,
    pub constitution: f32,
}

#[derive(Component)]
pub struct ComponentBodyHandle {
    pub handle: BodyHandle,
}
