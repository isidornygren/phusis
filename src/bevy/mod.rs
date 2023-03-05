use bevy::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;

use crate::{
    bevy::components::PhysicsWorldResource,
    quad_tree::QuadTree,
    shape::AABB,
    world::PhysicsWorld,
};

mod components;
#[cfg(feature = "bevy_debug")]
mod debug;
mod systems;

pub use components::{Collider, Collisions, Sensor};

#[derive(StageLabel)]
pub enum PhusisStage {
    Init,
    PreUpdate,
    Update,
    PostUpdate,
}

pub struct PhusisBevyPlugin;

impl Plugin for PhusisBevyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PhysicsWorldResource {
            physics_world: PhysicsWorld::new(QuadTree::new(0, AABB::new(-1000, -1000, 1000, 1000))),
        })
        .add_stage_after(
            CoreStage::PostUpdate,
            PhusisStage::Init,
            SystemStage::parallel(),
        )
        .add_stage_after(
            PhusisStage::Init,
            PhusisStage::PreUpdate,
            SystemStage::parallel(),
        )
        .add_stage_after(
            PhusisStage::PreUpdate,
            PhusisStage::Update,
            SystemStage::parallel(),
        )
        .add_stage_after(
            PhusisStage::Update,
            PhusisStage::PostUpdate,
            SystemStage::parallel(),
        )
        .add_system_to_stage(PhusisStage::Init, systems::on_body_change)
        .add_system_to_stage(PhusisStage::PreUpdate, systems::on_body_transform_change)
        .add_system_to_stage(PhusisStage::Update, systems::update_physics);

        #[cfg(feature = "bevy_debug")]
        app.add_plugin(ShapePlugin);
        app.add_plugin(debug::PhusisBevyDebugPlugin);
    }
}
