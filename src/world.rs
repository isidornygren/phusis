use std::cell::RefCell;
use std::mem;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::body::Body;
use crate::checks::check_collision;
use crate::collision::Collision;
use crate::quad_tree::{self, WrappedBody};
use crate::shape::AABB;
use crate::Vec2;

/**
 * Sets velocity in m/s
 */
fn resolve_collision(a: &mut Body, b: &mut Body, collision: &Collision) {
    if a.fixed && b.fixed {
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
    if a.fixed && b.fixed {
        return;
    }
    // High percentage = no penetration
    let percent = 0.5;
    // Allows penetration without jittering
    let k_slop = 0.01;

    let maximum = 0f32.max(collision.penetration_depth - k_slop);

    let correction_scalar = if b.fixed {
        maximum / a.inv_mass * percent
    } else if a.fixed {
        maximum / b.inv_mass * percent
    } else {
        maximum / (a.inv_mass + b.inv_mass) * percent
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
            quad_tree: quad_tree::QuadTree::new(0, AABB::new(-100f32, -100f32, 1000f32, 1000f32)),
            removed_indices: vec![],
        }
    }
}

impl PhysicsWorld {
    pub fn add_body(&mut self, body: Body) -> BodyHandle {
        if let Some(removed_index) = self.removed_indices.pop() {
            mem::replace(
                self.bodies.get_mut(removed_index).unwrap(),
                Arc::new(Mutex::new(body)),
            );
            return BodyHandle {
                index: removed_index,
            };
        }
        self.bodies.push(Arc::new(Mutex::new(body)));
        BodyHandle {
            index: self.bodies.len() - 1,
        }
    }

    pub fn remove_body(&mut self, handle: BodyHandle) {
        self.removed_indices.push(handle.index);
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
            // TODO: check surface friction
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
        self.quad_tree.clear();
        for body in &self.bodies {
            self.quad_tree.insert(Arc::clone(body));
        }
        let collisions = self.quad_tree.check_collisions();

        for collision in collisions {
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
