use bevy::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;
use phusis::{
    bevy::{ComponentBody, PhusisBevyPlugin},
    shape::{Circle, Shape},
};
use rand::prelude::*;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn add_bodies(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    for _ in 0..1000 {
        let x = rng.gen_range(-100..100) as f32;
        let y = rng.gen_range(-100..100) as f32;
        let radius = 32.0;

        commands
            .spawn(Transform::from_xyz(x, y, 1.0))
            .insert(ComponentBody {
                mass: 1.0,
                shape: Shape::Circle(Circle::new(radius)),
                constitution: 1.0,
            });
    }
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(PhusisBevyPlugin)
        .add_startup_system(setup)
        .add_startup_system(add_bodies)
        .run();
}
