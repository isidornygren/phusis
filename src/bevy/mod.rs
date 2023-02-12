use bevy::prelude::*;

use crate::world::PhysicsWorld;

use self::components::PhysicsWorldResource;

mod components;
#[cfg(feature = "bevy_debug")]
mod debug;
mod systems;

pub use components::{Collider, Sensor};

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
            physics_world: PhysicsWorld::default(),
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
        .add_system_to_stage(PhusisStage::Update, systems::update_physics);

        #[cfg(feature = "bevy_debug")]
        app.add_plugin(debug::PhusisBevyDebugPlugin);
    }
}
