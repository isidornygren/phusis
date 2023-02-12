use bevy::prelude::*;

use crate::{body::Body, Vec2};

use super::components::{Collider, ComponentBodyHandle, PhysicsWorldResource, Sensor};

pub fn on_body_change(
    mut commands: Commands,
    mut physics_world: ResMut<PhysicsWorldResource>,
    query: Query<(&Collider, &Transform, Option<&Sensor>, Entity), Added<Collider>>,
) {
    for (collider, transform, sensor, entity) in query.iter() {
        let handle = physics_world.physics_world.add_body(Body {
            shape: collider.shape.clone(),
            position: Vec2::new(transform.translation.x, transform.translation.y),
            fixed: collider.fixed,
            sensor: sensor.is_some(),
            ..default()
        });
        commands
            .entity(entity)
            .insert(ComponentBodyHandle { handle });
    }
}

// fn on_body_transform_change(
//     physics_world: ResMut<PhysicsWorldResource>,
//     query: Query<(&ComponentBodyHandle, &Transform), Changed<Transform>>,
// ) {
//     for (body_handle, transform) in query.iter() {
//         if let Some(body) = physics_world.physics_world.get_body(&body_handle.handle) {
//             body.lock().unwrap().position =
//                 phusis::Vec2::new(transform.translation.x, transform.translation.y);
//         }
//     }
// }

pub fn update_physics(
    time: Res<Time>,
    mut physics_world: ResMut<PhysicsWorldResource>,
    mut query: Query<(&ComponentBodyHandle, &mut Transform)>,
) {
    physics_world
        .physics_world
        .update_with_quad(time.delta_seconds());

    for (body_handle, mut transform) in query.iter_mut() {
        if let Some(body) = physics_world.physics_world.get_body(&body_handle.handle) {
            let borrowed_body = body.lock().unwrap();
            transform.translation =
                Vec3::new(borrowed_body.position.x, borrowed_body.position.y, 1.0);
        }
    }
}
