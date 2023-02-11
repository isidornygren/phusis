extern crate phusis;
extern crate stopwatch;

use bevy_prototype_lyon::prelude::ShapePlugin;
use phusis::body::Body;
use phusis::shape::{Circle, Shape};
use phusis::world::{BodyHandle, PhysicsWorld};

use bevy::prelude::*;

use rand::prelude::*;

#[derive(Resource)]
struct PhysicsWorldResource {
    physics_world: PhysicsWorld,
}

#[derive(Component)]
struct ComponentBody {
    shape: Shape,
    mass: f32,
    constitution: f32,
}

#[derive(Component)]
struct ComponentBodyHandle {
    handle: BodyHandle,
}

fn on_body_change(
    mut commands: Commands,
    mut physics_world: ResMut<PhysicsWorldResource>,
    query: Query<(&ComponentBody, &Transform, Entity), Added<ComponentBody>>,
) {
    for (body, transform, entity) in query.iter() {
        let handle = physics_world.physics_world.add_body(Body {
            shape: body.shape.clone(),
            position: phusis::Vec2::new(transform.translation.x, transform.translation.y),
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

fn update_physics(
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

fn add_bodies(mut commands: Commands) {
    use bevy_prototype_lyon::prelude::*;

    let mut rng = rand::thread_rng();

    for _ in 0..1000 {
        let x = rng.gen_range(-100..100) as f32;
        let y = rng.gen_range(-100..100) as f32;
        let radius = 32.0;

        let shape = shapes::RegularPolygon {
            sides: 24,
            feature: shapes::RegularPolygonFeature::Radius(radius),
            ..shapes::RegularPolygon::default()
        };

        commands
            .spawn(GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::CYAN),
                    outline_mode: StrokeMode::new(Color::BLACK, 10.0),
                },
                Transform::from_xyz(x, y, 1.0),
            ))
            .insert(ComponentBody {
                mass: 1.0,
                shape: Shape::Circle(Circle::new(radius)),
                constitution: 1.0,
            });
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .insert_resource(PhysicsWorldResource {
            physics_world: PhysicsWorld::default(),
        })
        .add_system(on_body_change)
        .add_system(update_physics)
        // .add_system(on_body_transform_change)
        .add_startup_system(setup)
        .add_startup_system(add_bodies)
        .run();
}
