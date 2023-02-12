use std::mem;
use std::sync::{Arc, Mutex};

use crate::body::Body;
use crate::collision::Collision;
use crate::quad_tree::{self, WrappedBody};
use crate::shape::AABB;
use crate::Vec2;

// High percentage = no penetration
const PENETRATION_PERCENTAGE: f32 = 0.5;
// Allows penetration without jittering
const K_SLOP: f32 = 0.01;

/**
 * Sets velocity in m/s
 */
fn resolve_collision(a: &mut Body, b: &mut Body, collision: &Collision) {
    if (a.fixed && b.fixed) || a.sensor || b.sensor {
        return;
    }
    let relative_velocity = &b.velocity - &a.velocity;
    let velocity_along_normal = relative_velocity.dot(&collision.normal);

    if velocity_along_normal > 0f32 {
        return;
    }

    let restitution = a.restitution.min(b.restitution);
    let mut impulse = -(1.0 + restitution) * velocity_along_normal;

    impulse = match (a.fixed, b.fixed) {
        (false, false) => impulse / (a.inv_mass + b.inv_mass),
        (false, true) => impulse / a.inv_mass,
        (true, false) => impulse / b.inv_mass,
        _ => impulse, // this will never happen
    };

    let impulse_vector = impulse * &collision.normal;

    if !a.fixed {
        a.velocity -= a.inv_mass * &impulse_vector;
    }
    if !b.fixed {
        b.velocity += b.inv_mass * &impulse_vector;
    }
}

fn correct_position(a: &mut Body, b: &mut Body, collision: &Collision) {
    if a.fixed && b.fixed || a.sensor || b.sensor {
        return;
    }

    let maximum = 0f32.max(collision.penetration_depth - K_SLOP);

    let correction_scalar = if b.fixed {
        maximum / a.inv_mass * PENETRATION_PERCENTAGE
    } else if a.fixed {
        maximum / b.inv_mass * PENETRATION_PERCENTAGE
    } else {
        maximum / (a.inv_mass + b.inv_mass) * PENETRATION_PERCENTAGE
    };

    let correction = &collision.normal * correction_scalar;
    if !a.fixed {
        a.position -= a.inv_mass * &correction;
    }
    if !b.fixed {
        b.position += b.inv_mass * &correction;
    }
}

#[derive(Debug)]
pub struct BodyHandle {
    index: usize,
}

pub struct PhysicsWorld {
    bodies: Vec<WrappedBody>,
    removed_indices: Vec<usize>,
    quad_tree: quad_tree::QuadTree,
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self {
            bodies: vec![],
            quad_tree: quad_tree::QuadTree::new(0, AABB::new(-1000f32, -1000f32, 1000f32, 1000f32)),
            removed_indices: vec![],
        }
    }
}

impl PhysicsWorld {
    pub fn add_body(&mut self, body: Body) -> BodyHandle {
        let body_mutex = Arc::new(Mutex::new(body));

        if let Some(removed_index) = self.removed_indices.pop() {
            mem::replace(
                self.bodies.get_mut(removed_index).unwrap(),
                Arc::clone(&body_mutex),
            );
            return BodyHandle {
                index: removed_index,
            };
        }
        self.bodies.push(Arc::clone(&body_mutex));
        self.quad_tree.insert(body_mutex);
        BodyHandle {
            index: self.bodies.len() - 1,
        }
    }

    pub fn remove_body(&mut self, handle: BodyHandle) {
        self.removed_indices.push(handle.index);
        let body_mutex = self.bodies.get(handle.index).unwrap();
        self.quad_tree.remove(body_mutex);
    }

    pub fn get_body(&self, handle: &BodyHandle) -> Option<&WrappedBody> {
        self.bodies.get(handle.index)
    }

    pub fn get_body_mut(&mut self, handle: BodyHandle) -> Option<&mut WrappedBody> {
        self.bodies.get_mut(handle.index)
    }

    fn calc_velocity(&mut self, dt: f32) {
        // Update position of bodies based on velocity
        for body_mutex in &mut self.bodies {
            // Apply force in body
            let mut body_mut = body_mutex.lock().unwrap();
            // TODO: Fix force code
            // this is not really using any fancy physics, it's just me (???!!!)
            let linear_acceleration = &body_mut.force / body_mut.mass;
            body_mut.velocity = &body_mut.velocity + linear_acceleration * dt;
            // Force has been applied, reset it in body
            body_mut.force = Vec2::new(0f32, 0f32);

            // Apply friction based on surface
            let friction_val = (body_mut.friction * &body_mut.velocity) * dt;
            body_mut.velocity -= friction_val;

            if body_mut.velocity.abs() < Vec2::new(0.1, 0.1) {
                body_mut.velocity = Vec2::new(0f32, 0f32);
            }

            body_mut.position = &body_mut.position + &body_mut.velocity * dt;
        }
    }

    pub fn update_with_quad(&mut self, dt: f32) {
        self.calc_velocity(dt);
        let collisions = self.quad_tree.check_collisions();

        println!("Collisions: {}", collisions.len());

        for collision in collisions {
            let a_sensor_or_fixed = collision
                .a
                .lock()
                .map_or(false, |a_body| a_body.fixed || a_body.sensor);
            let b_sensor_or_fixed = collision
                .b
                .lock()
                .map_or(false, |b_body| b_body.fixed || b_body.sensor);

            if !a_sensor_or_fixed {
                self.quad_tree.remove(&collision.a);
            }
            if !b_sensor_or_fixed {
                self.quad_tree.remove(&collision.b);
            }

            resolve_collision(
                &mut collision.a.lock().unwrap(),
                &mut collision.b.lock().unwrap(),
                &collision,
            );
            correct_position(
                &mut collision.a.lock().unwrap(),
                &mut collision.b.lock().unwrap(),
                &collision,
            );

            if !a_sensor_or_fixed {
                self.quad_tree.insert(collision.a);
            }
            if !b_sensor_or_fixed {
                self.quad_tree.insert(collision.b);
            }
        }
    }

    #[must_use]
    pub fn get_quad_tree_aabb(&self) -> Vec<AABB> {
        self.quad_tree.get_node_aabb()
    }

    // pub fn update(&mut self, dt: f32) {
    //     self.calc_velocity(dt);
    //     // Resolve collision for body pairs
    //     for (i, a) in self.bodies.iter().enumerate() {
    //         for b in &self.bodies[(i + 1)..] {
    //             // The collision checking stage can definitely be parallelised (probably)
    //             let resolution = check_collision(a, b);
    //             if let Some(collision) = resolution {
    //                 resolve_collision(&mut a.borrow_mut(), &mut b.borrow_mut(), &collision);
    //                 correct_position(&mut a.borrow_mut(), &mut b.borrow_mut(), &collision);
    //             }
    //         }
    //     }
    // }
}
