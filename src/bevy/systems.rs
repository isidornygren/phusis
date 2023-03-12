use bevy::prelude::*;

use super::components::PhysicsWorldResource;
use crate::{
    bevy::components::{Collider, Collisions, ComponentBodyHandle},
    body::Body,
};

pub fn on_body_change(
    mut commands: Commands,
    mut physics_world: ResMut<PhysicsWorldResource>,
    query: Query<(&Collider, &Transform, Entity), Added<Collider>>,
) {
    for (collider, transform, entity) in query.iter() {
        let handle = physics_world.physics_world.add_body(Body {
            shape: collider.shape.clone(),
            position: crate::Vec2::new(transform.translation.x, transform.translation.y),
            fixed: collider.fixed,
            sensor: collider.sensor,
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
        physics_world
            .physics_world
            .update(&body_handle.handle, |body| {
                body.position = crate::Vec2::new(transform.translation.x, transform.translation.y);
            });
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
        if let Ok(mut collision_entity) = collisions_q.get_mut(
            physics_world
                .physics_world
                .get_body(collision.pair.a)
                .unwrap()
                .entity,
        ) {
            collision_entity.entities.push(
                physics_world
                    .physics_world
                    .get_body(collision.pair.b)
                    .unwrap()
                    .entity,
            );
        }
    }

    for (body_handle, mut transform) in query.iter_mut() {
        if let Some(body) = physics_world.physics_world.get_body(body_handle.handle) {
            if transform.translation.x != body.position.x
                && transform.translation.y != body.position.y
            {
                transform.translation = Vec3::new(body.position.x, body.position.y, 1.0);
            }
        }
    }
}
