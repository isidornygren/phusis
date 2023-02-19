use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use phusis::{
    bevy::{Collider, PhusisBevyPlugin},
    shape::{Circle, Shape},
};
use rand::prelude::*;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn add_bodies(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    for _ in 0..10000 {
        let x = rng.gen_range(-100..100) as f32;
        let y = rng.gen_range(-100..100) as f32;
        let radius = 3.0;

        commands
            .spawn(Transform::from_xyz(x, y, 1.0))
            .insert(Collider {
                mass:         1.0,
                shape:        Shape::Circle(Circle::new(radius)),
                constitution: 1.0,
                fixed:        false,
            });
    }

    // for _ in 0..100 {
    //     let x = rng.gen_range(-100..100) as f32;
    //     let y = rng.gen_range(-100..100) as f32;
    //     let radius = 32.0;

    //     commands
    //         .spawn(Transform::from_xyz(x, y, 1.0))
    //         .insert(Collider {
    //             mass: 1.0,
    //             shape: Shape::AABB(AABB::new(0.0, 0.0, radius * 2.0, radius * 2.0)),
    //             constitution: 1.0,
    //             fixed: true,
    //         });
    // }

    // for _ in 0..100 {
    //     let x = rng.gen_range(-100..100) as f32;
    //     let y = rng.gen_range(-100..100) as f32;
    //     let radius = 32.0;

    //     commands
    //         .spawn(Transform::from_xyz(x, y, 1.0))
    //         .insert(Collider {
    //             mass: 1.0,
    //             shape: Shape::Circle(Circle::new(radius)),
    //             constitution: 1.0,
    //             fixed: false,
    //         })
    //         .insert(Sensor);
    // }
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(PhusisBevyPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_startup_system(add_bodies)
        .run();
}
