use nalgebra::Vector2;

// use physics::aabb::AABB;
use crate::body::Body;
use crate::collision::{Collision, Manifold};
use crate::shape::Circle;

fn distance_squared(vec: &Vector2<f32>) -> f32 {
    (vec.x).powf(2f32) + (vec.y).powf(2f32)
}

/* pub fn aabb_vs_aabb(a: &AABB, b: &AABB) -> bool {
    if a.max.x < b.min.y || a.min.x > b.max.x {
        return false;
    }
    if a.max.y < b.min.y || a.min.y > b.max.y {
        return false;
    }
    return true;
} */

/* pub fn circle_vs_circle(a: &Circle, b: &Circle) -> bool {
    let r = (a.radius + b.radius).powf(2f32);
    return r < (a.position.x + b.position.x).powf(2f32) + (a.position.y + b.position.y).powf(2f32);
} */

pub fn circle_vs_circle(a: &Body, b: &Body) -> Option<Collision> {
    let normal = b.position - a.position;

    let radius = (a.shape.get_radius() + b.shape.get_radius()).powf(2f32);

    let distance_sqr = distance_squared(&normal);

    if distance_sqr > radius {
        return None;
    }
    let distance = distance_sqr.sqrt();

    if distance != 0f32 {
        return Some(Collision {
            penetration_depth: (a.shape.get_radius() + b.shape.get_radius()) - distance,
            normal: normal / distance,
        });
    } else {
        // Circles are on the same position
        // Choose random (but consistent) values
        return Some(Collision {
            penetration_depth: a.shape.get_radius(),
            normal: Vector2::new(1f32, 0f32),
        });
    }
}

/* pub fn AABB_vs_AABB_manifold(manifold: &Manifold<AABB, AABB>) -> Option<Collision> {
    let normal = manifold.b.position - manifold.a.position;

    let a_extent_x = (manifold.a.shape.max.x - manifold.b.shape.min.x) / 2f32;
    let b_extent_x = (manifold.b.shape.max.x - manifold.a.shape.min.x) / 2f32;

    let x_overlap = a_extent_x + b_extent_x - normal.x.abs();
    if x_overlap > 0f32 {
        let a_extent_y = (manifold.a.shape.max.y - manifold.b.shape.min.y) / 2f32;
        let b_extent_y = (manifold.b.shape.max.y - manifold.a.shape.min.y) / 2f32;

        let y_overlap = a_extent_y + b_extent_y - normal.y.abs();
        if y_overlap > 0f32 {
            if x_overlap < y_overlap {
                return Some(Collision {
                    normal: if normal.x < 0f32 {
                        Vector2::new(-1f32, 0f32)
                    } else {
                        Vector2::new(1f32, 0f32)
                    },
                    penetration_depth: x_overlap,
                });
            } else {
                return Some(Collision {
                    normal: if normal.y < 0f32 {
                        Vector2::new(0f32, -1f32)
                    } else {
                        Vector2::new(0f32, 1f32)
                    },
                    penetration_depth: x_overlap,
                });
            }
        }
    }
    return None;
}

pub fn AABB_vs_circle(manifold: &Manifold<AABB, Circle>) -> Option<Collision> {
    unimplemented!();
} */
