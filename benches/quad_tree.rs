use bevy::prelude::Entity;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use phusis::{
    body::Body,
    shape::{Circle, Shape},
    world::PhysicsWorld,
    Vec2,
};
use rand::prelude::*;

const BODIES: u32 = 200;

fn quad_tree_bench(c: &mut Criterion) {
    c.bench_function("insertion", |b| {
        let mut physics_world = PhysicsWorld::default();

        b.iter(|| {
            physics_world.add_body(Body::new(
                black_box(1f32),
                black_box(1f32),
                Shape::Circle(Circle::new(8f32)),
                Vec2::new(1f32, 1f32),
                black_box(false),
                black_box(false),
                Entity::from_raw(0),
            ))
        })
    });
    c.bench_function("collision_update 200", |b| {
        let mut physics_world = PhysicsWorld::default();
        let mut rng = rand::thread_rng();

        for _ in 0..BODIES {
            let x = rng.gen_range(0..100) as f32;
            let y = rng.gen_range(0..100) as f32;

            let _new_body = physics_world.add_body(Body::new(
                1f32,
                1f32,
                Shape::Circle(Circle::new(10f32)),
                Vec2::new(x, y),
                false,
                false,
                Entity::from_raw(0),
            ));
        }
        b.iter(|| {
            physics_world
                .quad_tree
                .check_collisions(&physics_world.bodies)
        })
    });

    c.bench_function("update 200", |b| {
        let mut physics_world = PhysicsWorld::default();
        let mut rng = rand::thread_rng();

        for _ in 0..BODIES {
            let x = rng.gen_range(0..100) as f32;
            let y = rng.gen_range(0..100) as f32;

            let _new_body = physics_world.add_body(Body::new(
                1f32,
                1f32,
                Shape::Circle(Circle::new(10f32)),
                Vec2::new(x, y),
                false,
                false,
                Entity::from_raw(0),
            ));
        }
        b.iter(|| physics_world.update_with_quad(black_box(1f32 / 60f32)))
    });
}

criterion_group!(benches, quad_tree_bench);
criterion_main!(benches);
