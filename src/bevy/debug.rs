use bevy::prelude::*;

use super::components::Collider;
use crate::shape::Shape;

fn debug_physics(
    mut commands: Commands,
    query: Query<(&Collider, &Transform, Entity), Changed<Collider>>,
) {
    use bevy_prototype_lyon::prelude::*;

    for (body, transform, entity) in query.iter() {
        let color = match (body.fixed, body.sensor) {
            (true, true) => Color::GREEN,
            (true, false) => Color::BLUE,
            (false, true) => Color::YELLOW,
            (false, false) => Color::RED,
        };

        let scale = 1.0; // 6.0; // transform.scale.x;

        match &body.shape {
            Shape::Circle(circle) => {
                let shape = shapes::RegularPolygon {
                    sides: 24,
                    feature: shapes::RegularPolygonFeature::Radius(circle.radius / scale),
                    ..default()
                };
                commands.entity(entity).insert(GeometryBuilder::build_as(
                    &shape,
                    DrawMode::Outlined {
                        fill_mode:    FillMode::color(Color::rgba(0.0, 0.0, 0.0, 0.0)),
                        outline_mode: StrokeMode::new(color, 1.0 / scale),
                    },
                    *transform,
                ));
            },
            Shape::Rect(rect) => {
                let shape = shapes::Rectangle {
                    extents: Vec2::new(rect.x / scale, rect.y / scale),
                    ..default()
                };
                commands.entity(entity).insert(GeometryBuilder::build_as(
                    &shape,
                    DrawMode::Outlined {
                        fill_mode:    FillMode::color(Color::rgba(0.0, 0.0, 0.0, 0.0)),
                        outline_mode: StrokeMode::new(color, 1.0 / scale),
                    },
                    *transform,
                ));
            },
        };
    }
}

pub struct PhusisBevyDebugPlugin;

impl Plugin for PhusisBevyDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(debug_physics);
    }
}
