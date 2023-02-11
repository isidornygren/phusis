use bevy::prelude::*;

use crate::shape::Shape;

use super::components::ComponentBody;

fn debug_physics(
    mut commands: Commands,
    query: Query<(&ComponentBody, &Transform, Entity), Changed<ComponentBody>>,
) {
    use bevy_prototype_lyon::prelude::*;

    for (body, transform, entity) in query.iter() {
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
                        outline_mode: StrokeMode::new(Color::GREEN, 1.0),
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
                        outline_mode: StrokeMode::new(Color::GREEN, 1.0),
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
