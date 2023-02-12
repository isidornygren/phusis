use bevy::prelude::*;

use crate::{bevy::components::Sensor, shape::Shape};

use super::components::Collider;

fn debug_physics(
    mut commands: Commands,
    query: Query<(&Collider, &Transform, Option<&Sensor>, Entity), Changed<Collider>>,
) {
    use bevy_prototype_lyon::prelude::*;

    for (body, transform, sensor, entity) in query.iter() {
        let color = match (body.fixed, sensor.is_some()) {
            (true, true) => Color::GREEN,
            (true, false) => Color::BLUE,
            (false, true) => Color::YELLOW,
            (false, false) => Color::RED,
        };

        match &body.shape {
            Shape::Circle(circle) => {
                let shape = shapes::RegularPolygon {
                    sides: 24,
                    feature: shapes::RegularPolygonFeature::Radius(circle.radius),
                    ..default()
                };
                commands.entity(entity).insert(GeometryBuilder::build_as(
                    &shape,
                    DrawMode::Outlined {
                        fill_mode: FillMode::color(Color::rgba(0.0, 0.0, 0.0, 0.0)),
                        outline_mode: StrokeMode::new(color, 1.0),
                    },
                    *transform,
                ));
            }
            Shape::AABB(aabb) => {
                let shape = shapes::Rectangle {
                    extents: Vec2::new(aabb.get_width(), aabb.get_height()),
                    ..default()
                };
                commands.entity(entity).insert(GeometryBuilder::build_as(
                    &shape,
                    DrawMode::Outlined {
                        fill_mode: FillMode::color(Color::rgba(0.0, 0.0, 0.0, 0.0)),
                        outline_mode: StrokeMode::new(color, 1.0),
                    },
                    *transform,
                ));
            }
        };
    }
}

pub struct PhusisBevyDebugPlugin;

impl Plugin for PhusisBevyDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(debug_physics);
    }
}
