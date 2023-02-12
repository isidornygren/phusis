use crate::quad_tree::WrappedBody;
use crate::world::BodyHandle;
use crate::Vec2;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use crate::body::Body;
use crate::collision::Collision;
use crate::shape::{Circle, Shape, AABB};

fn distance_squared(vec: &Vec2) -> f32 {
    (vec.x).powf(2f32) + (vec.y).powf(2f32)
}

pub fn check_collision(a: &WrappedBody, b: &WrappedBody) -> Option<Collision> {
    let (a_shape, a_position) = {
        let a_body = a.lock().unwrap();
        (&a_body.shape.clone(), &a_body.position.clone())
    };
    let (b_shape, b_position) = {
        let b_body = b.lock().unwrap();
        (&b_body.shape.clone(), &b_body.position.clone())
    };

    match (a_shape, b_shape) {
        (Shape::Circle(a_circle), Shape::Circle(b_circle)) => {
            circle_vs_circle(a, b, a_circle, b_circle, a_position, b_position)
        }
        (Shape::AABB(a_aabb), Shape::AABB(b_aabb)) => {
            aabb_vs_aabb(a, b, a_aabb, b_aabb, a_position, b_position)
        }
        (Shape::Circle(circle), Shape::AABB(aabb)) => {
            aabb_vs_circle(b, a, aabb, circle, a_position, b_position)
        }
        (Shape::AABB(aabb), Shape::Circle(circle)) => {
            aabb_vs_circle(a, b, aabb, circle, a_position, b_position)
        }
    }
}

pub fn aabb_vs_aabb(
    a_mutex: &WrappedBody,
    b_mutex: &WrappedBody,
    a_aabb: &AABB,
    b_aabb: &AABB,
    a_position: &Vec2,
    b_position: &Vec2,
) -> Option<Collision> {
    let pos_diff = b_position - a_position;

    let penetration = (b_aabb.half.clone() + a_aabb.half.clone()) - pos_diff.abs();
    if penetration.x <= 0f32 || penetration.y <= 0f32 {
        return None;
    }
    if penetration.x < penetration.y {
        let sign_x = pos_diff.x.signum();
        return Some(Collision {
            penetration_depth: penetration.x * sign_x,
            normal: Vec2::new(sign_x, 0f32),
            a: a_mutex.clone(),
            b: b_mutex.clone(),
        });
    }
    let sign_y = pos_diff.y.signum();
    Some(Collision {
        penetration_depth: penetration.y * sign_y,
        normal: Vec2::new(0f32, sign_y),
        a: a_mutex.clone(),
        b: b_mutex.clone(),
    })
}

pub fn circle_vs_circle(
    a_mutex: &WrappedBody,
    b_mutex: &WrappedBody,
    a_circle: &Circle,
    b_circle: &Circle,
    a_position: &Vec2,
    b_position: &Vec2,
) -> Option<Collision> {
    let normal = b_position - a_position;

    let radius = (a_circle.radius + b_circle.radius).powf(2f32);

    let distance_sqr = distance_squared(&normal);

    if distance_sqr > radius {
        return None;
    }
    let distance = distance_sqr.sqrt();

    if distance != 0f32 {
        return Some(Collision {
            penetration_depth: (a_circle.radius + b_circle.radius) - distance,
            normal: normal / distance,
            a: a_mutex.clone(),
            b: b_mutex.clone(),
        });
    }
    // Circles are on the same position
    // Choose random (but consistent) values
    Some(Collision {
        penetration_depth: a_circle.radius,
        normal: Vec2::new(1f32, 0f32),
        a: a_mutex.clone(),
        b: b_mutex.clone(),
    })
}

pub fn aabb_vs_circle(
    a_mutex: &WrappedBody,
    b_mutex: &WrappedBody,
    a_aabb: &AABB,
    b_circle: &Circle,
    a_position: &Vec2,
    b_position: &Vec2,
) -> Option<Collision> {
    let normal = b_position - a_position;

    let x_extent = a_aabb.get_width() / 2f32;
    let y_extent = a_aabb.get_height() / 2f32;

    let mut closest = Vec2::new(
        f32::clamp(normal.x, -x_extent, x_extent),
        f32::clamp(normal.y, -y_extent, y_extent),
    );

    let mut inside = false;

    // Circle is inside the AABB
    if normal == closest {
        inside = true;
        // finds the closest axis
        if normal.x.abs() > normal.y.abs() {
            if closest.x > 0f32 {
                closest.x = x_extent;
            } else {
                closest.x = -x_extent;
            }
        } else if closest.y > 0f32 {
            closest.y = y_extent;
        } else {
            closest.y = -y_extent;
        }
    }

    let distance = distance_squared(&(&normal - &closest));
    let radius = b_circle.radius;

    // Return none if radius is shorter than distance to closest
    // point and circle not inside AABB
    if distance > (radius * radius) && !inside {
        return None;
    }

    let distance_sqr = distance.sqrt();

    if inside {
        Some(Collision {
            penetration_depth: (radius - distance_sqr),
            normal: &normal / radius,
            a: a_mutex.clone(),
            b: b_mutex.clone(),
        })
    } else {
        Some(Collision {
            penetration_depth: (radius - distance_sqr),
            normal: &normal / distance,
            a: a_mutex.clone(),
            b: b_mutex.clone(),
        })
    }
}
