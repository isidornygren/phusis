use crate::{
    collision::Contact,
    shape::{Circle, AABB},
    Vec2,
};

fn distance_squared(vec: Vec2<f32>) -> f32 {
    (vec.x).powi(2) + (vec.y).powi(2)
}

pub fn aabb_vs_aabb(a: &AABB<f32>, b: &AABB<f32>) -> Option<Contact<f32>> {
    let pos_diff = b.min - a.min;

    let penetration =
        (((b.max - b.min) / 2.0 + b.min) + ((a.max - a.min) / 2.0 + a.min)) - pos_diff.abs();
    if penetration.x <= 0f32 || penetration.y <= 0f32 {
        return None;
    }
    if penetration.x < penetration.y {
        let sign_x = pos_diff.x.signum();
        return Some(Contact {
            penetration_depth: penetration.x * sign_x,
            normal:            Vec2::new(sign_x, 0f32),
        });
    }
    let sign_y = pos_diff.y.signum();
    Some(Contact {
        penetration_depth: penetration.y * sign_y,
        normal:            Vec2::new(0f32, sign_y),
    })
}

pub fn rect_vs_rect(
    a: &Vec2<f32>,
    b: &Vec2<f32>,
    a_position: &Vec2<f32>,
    b_position: &Vec2<f32>,
) -> Option<Contact<f32>> {
    let pos_diff = *b_position - *a_position;

    let penetration = ((*b / 2.0 + *b_position) + (*a / 2.0 + *a_position)) - pos_diff.abs();
    if penetration.x <= 0f32 || penetration.y <= 0f32 {
        return None;
    }
    if penetration.x < penetration.y {
        let sign_x = pos_diff.x.signum();
        return Some(Contact {
            penetration_depth: penetration.x * sign_x,
            normal:            Vec2::new(sign_x, 0f32),
        });
    }
    let sign_y = pos_diff.y.signum();
    Some(Contact {
        penetration_depth: penetration.y * sign_y,
        normal:            Vec2::new(0f32, sign_y),
    })
}

pub fn circle_vs_circle(
    a_circle: &Circle,
    b_circle: &Circle,
    a_position: Vec2<f32>,
    b_position: Vec2<f32>,
) -> Option<Contact<f32>> {
    let distance = b_position - a_position;

    let radius = (a_circle.radius + b_circle.radius).powi(2);

    let distance_sqr = distance_squared(distance);

    if distance_sqr > radius {
        return None;
    }

    let distance_sqrt = distance_sqr.sqrt();

    if distance_sqrt != 0f32 {
        return Some(Contact {
            penetration_depth: (a_circle.radius + b_circle.radius) - distance_sqrt,
            normal:            distance / distance_sqrt,
        });
    }
    // Circles are on the same position
    // Choose random (but consistent) values
    Some(Contact {
        penetration_depth: a_circle.radius.min(b_circle.radius),
        normal:            Vec2::new(1f32, 0f32),
    })
}

pub fn rect_vs_circle(
    a_rect: &Vec2<f32>,
    b_circle: &Circle,
    a_position: &Vec2<f32>,
    b_position: &Vec2<f32>,
) -> Option<Contact<f32>> {
    let normal = *b_position - *a_position;

    let x_extent = a_rect.x / 2f32;
    let y_extent = a_rect.y / 2f32;

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

    let distance = distance_squared(normal - closest);
    let radius = b_circle.radius;

    // Return none if radius is shorter than distance to closest
    // point and circle not inside AABB
    if distance > (radius * radius) && !inside {
        return None;
    }

    let distance_sqr = distance.sqrt();

    if inside {
        Some(Contact {
            penetration_depth: (radius - distance_sqr),
            normal:            normal / radius,
        })
    } else {
        Some(Contact {
            penetration_depth: (radius - distance_sqr),
            normal:            normal / distance,
        })
    }
}
