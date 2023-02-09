use crate::Vec2;

use std::cell::RefCell;
use std::rc::Rc;

use crate::body::Body;
use crate::collision::Collision;
use crate::shape::ShapeKind;

fn distance_squared(vec: &Vec2) -> f32 {
    (vec.x).powf(2f32) + (vec.y).powf(2f32)
}

pub fn check_collision(a: &Rc<RefCell<Body>>, b: &Rc<RefCell<Body>>) -> Option<Collision> {
    match (a.borrow().shape.get_kind(), b.borrow().shape.get_kind()) {
        (ShapeKind::Circle, ShapeKind::Circle) => circle_vs_circle(a, b),
        (ShapeKind::AABB, ShapeKind::AABB) => aabb_vs_aabb(a, b),
        (ShapeKind::Circle, ShapeKind::AABB) => aabb_vs_circle(b, a),
        (ShapeKind::AABB, ShapeKind::Circle) => aabb_vs_circle(a, b),
    }
}

pub fn aabb_vs_aabb(a: &Rc<RefCell<Body>>, b: &Rc<RefCell<Body>>) -> Option<Collision> {
    let a_borrowed = a.borrow();
    let b_borrowed = b.borrow();

    let pos_diff = &b_borrowed.position - &a_borrowed.position;

    let penetration = (b_borrowed.get_aabb().half + a_borrowed.get_aabb().half) - pos_diff.abs();
    if penetration.x <= 0f32 || penetration.y <= 0f32 {
        return None;
    }
    if penetration.x < penetration.y {
        let sign_x = pos_diff.x.signum();
        return Some(Collision {
            penetration_depth: penetration.x * sign_x,
            normal: Vec2::new(sign_x, 0f32),
            a: Rc::clone(a),
            b: Rc::clone(b),
        });
    }
    let sign_y = pos_diff.y.signum();
    Some(Collision {
        penetration_depth: penetration.y * sign_y,
        normal: Vec2::new(0f32, sign_y),
        a: Rc::clone(a),
        b: Rc::clone(b),
    })
}

pub fn circle_vs_circle(a: &Rc<RefCell<Body>>, b: &Rc<RefCell<Body>>) -> Option<Collision> {
    let a_borrowed = a.borrow();
    let b_borrowed = b.borrow();
    let normal = &b_borrowed.position - &a_borrowed.position;

    let radius = (a_borrowed.shape.get_radius() + b_borrowed.shape.get_radius()).powf(2f32);

    let distance_sqr = distance_squared(&normal);

    if distance_sqr > radius {
        return None;
    }
    let distance = distance_sqr.sqrt();

    if distance != 0f32 {
        return Some(Collision {
            penetration_depth: (a_borrowed.shape.get_radius() + b_borrowed.shape.get_radius())
                - distance,
            normal: normal / distance,
            a: Rc::clone(a),
            b: Rc::clone(b),
        });
    }
    // Circles are on the same position
    // Choose random (but consistent) values
    Some(Collision {
        penetration_depth: a_borrowed.shape.get_radius(),
        normal: Vec2::new(1f32, 0f32),
        a: Rc::clone(a),
        b: Rc::clone(b),
    })
}

pub fn aabb_vs_circle(a: &Rc<RefCell<Body>>, b: &Rc<RefCell<Body>>) -> Option<Collision> {
    let a_borrowed = a.borrow();
    let b_borrowed = b.borrow();

    let normal = &b_borrowed.position - &a_borrowed.position;
    let mut closest = normal.clone();

    let x_extent = a_borrowed.get_aabb().get_width() / 2f32;
    let y_extent = a_borrowed.get_aabb().get_height() / 2f32;

    closest.x = f32::clamp(closest.x, -x_extent, x_extent);
    closest.y = f32::clamp(closest.y, -y_extent, y_extent);

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
    let radius = b_borrowed.shape.get_radius();

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
            a: Rc::clone(a),
            b: Rc::clone(b),
        })
    } else {
        Some(Collision {
            penetration_depth: (radius - distance_sqr),
            normal: &normal / distance,
            a: Rc::clone(a),
            b: Rc::clone(b),
        })
    }
}
