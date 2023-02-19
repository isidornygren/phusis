use std::sync::Arc;

use bevy::prelude::*;

use crate::{body::Body, Vec2};

use super::components::{
    Collider, Collisions, ComponentBodyHandle, KinematicController, PhysicsWorldResource, Sensor,
};

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
            entity,
            ..default()
        });
        commands
            .entity(entity)
            .insert(ComponentBodyHandle { handle });
    }
}

pub fn on_body_transform_change(
    mut physics_world: ResMut<PhysicsWorldResource>,
    query: Query<(&ComponentBodyHandle, &Transform), Changed<Transform>>,
) {
    for (body_handle, transform) in query.iter() {
        let maybe_body = physics_world
            .physics_world
            .get_body(&body_handle.handle)
            .map(Arc::clone);

        if let Some(body) = maybe_body {
            physics_world.physics_world.quad_tree.remove(&body);
            body.lock().unwrap().position =
                Vec2::new(transform.translation.x, transform.translation.y);
            physics_world.physics_world.quad_tree.insert(body);
        };
    }
}

pub fn update_physics(
    time: Res<Time>,
    mut physics_world: ResMut<PhysicsWorldResource>,
    mut query: Query<(&ComponentBodyHandle, &mut Transform)>,
    mut collisions_q: Query<&mut Collisions>,
) {
    let collisions = physics_world
        .physics_world
        .update_with_quad(time.delta_seconds());

    for mut collision in collisions_q.iter_mut() {
        collision.entities.clear();
    }

    for collision in collisions {
        if let Ok(mut collision_entity) = collisions_q.get_mut(collision.a.lock().unwrap().entity) {
            collision_entity
                .entities
                .push(collision.b.lock().unwrap().entity);
        }
    }

    for (body_handle, mut transform) in query.iter_mut() {
        if let Some(body) = physics_world.physics_world.get_body(&body_handle.handle) {
            let borrowed_body = body.lock().unwrap();
            if transform.translation.x != borrowed_body.position.x
                && transform.translation.y != borrowed_body.position.y
            {
                transform.translation =
                    Vec3::new(borrowed_body.position.x, borrowed_body.position.y, 1.0);
            }
        }
    }
}
