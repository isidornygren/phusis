use bevy::prelude::Entity;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use phusis::{
    body::Body,
    shape::{Circle, Shape, AABB},
    world::{broad::BroadPhase, PhysicsWorld},
    QuadTree,
    Vec2,
};
use rand::prelude::*;

fn quad_tree_bench(c: &mut Criterion) {
    c.bench_function("insertion", |b| {
        let mut physics_world =
            PhysicsWorld::new(QuadTree::new(0, AABB::new(-1000, -1000, 1000, 1000)));

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
    c.bench_function("deletion", |b| {
        // b.iter_batched(|| data.clone(), |mut data| sort(&mut data), BatchSize::SmallInput)

        b.iter_batched(
            || {
                let mut physics_world =
                    PhysicsWorld::new(QuadTree::new(0, AABB::new(-1000, -1000, 1000, 1000)));
                let handle = physics_world.add_body(Body::new(
                    black_box(1f32),
                    black_box(1f32),
                    Shape::Circle(Circle::new(8f32)),
                    Vec2::new(1f32, 1f32),
                    black_box(false),
                    black_box(false),
                    Entity::from_raw(0),
                ));

                (physics_world, handle)
            },
            |(mut physics_world, handle)| {
                physics_world.remove_body(handle);
            },
            criterion::BatchSize::SmallInput,
        )
    });

    c.bench_function("collision_update 200", |b| {
        let mut physics_world =
            PhysicsWorld::new(QuadTree::new(0, AABB::new(-1000, -1000, 1000, 1000)));
        let mut rng = rand::thread_rng();

        for _ in 0..200 {
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
        b.iter(|| physics_world.broad_phase.check_collisions())
    });

    c.bench_function("update 200", |b| {
        let mut physics_world =
            PhysicsWorld::new(QuadTree::new(0, AABB::new(-1000, -1000, 1000, 1000)));
        let mut rng = rand::thread_rng();

        for _ in 0..200 {
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

    c.bench_function("collision_update 2000", |b| {
        let mut physics_world =
            PhysicsWorld::new(QuadTree::new(0, AABB::new(-1000, -1000, 1000, 1000)));
        let mut rng = rand::thread_rng();

        for _ in 0..2000 {
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
        b.iter(|| physics_world.broad_phase.check_collisions())
    });

    c.bench_function("update 2000", |b| {
        let mut physics_world =
            PhysicsWorld::new(QuadTree::new(0, AABB::new(-1000, -1000, 1000, 1000)));
        let mut rng = rand::thread_rng();

        for _ in 0..2000 {
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
