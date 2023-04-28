use bevy::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;

mod components;
#[cfg(feature = "bevy_debug")]
mod debug;
mod systems;

pub use components::{Collider, Collisions};

use self::components::PhysicsWorldResource;
use crate::{shape::AABB, world::PhysicsWorld, QuadTree};

pub struct PhusisBevyPlugin;

impl Plugin for PhusisBevyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PhysicsWorldResource {
            physics_world: PhysicsWorld::new(QuadTree::new(
                0,
                AABB::new(-5000, -5000, 10000, 10000),
            )),
        })
        .add_system(systems::on_body_change)
        .add_system(systems::update_physics)
        .add_system(
            systems::on_body_transform_change
                .before(systems::update_physics)
                .after(systems::on_body_change),
        );

        #[cfg(feature = "bevy_debug")]
        app.add_plugin(ShapePlugin);
        app.add_plugin(debug::PhusisBevyDebugPlugin);
    }
}
