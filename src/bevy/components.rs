use bevy::prelude::*;
use generational_arena::Index;

use crate::{quad_tree::QuadTree, shape::Shape, world::PhysicsWorld};

#[derive(Resource)]
pub struct PhysicsWorldResource {
    pub physics_world: PhysicsWorld<QuadTree<Index>>,
}
#[derive(Component)]
pub struct Collider {
    pub shape:        Shape,
    pub mass:         f32,
    pub constitution: f32,
    pub fixed:        bool,
    pub sensor:       bool,
}

#[derive(Component, Default)]
pub struct Collisions {
    pub entities: Vec<Entity>,
}

#[derive(Component)]
pub struct Fixed;

#[derive(Component)]
pub struct ComponentBodyHandle {
    pub handle: Index,
}

#[derive(Component)]
pub struct KinematicController {
    pub translation: Vec2,
}
