use std::cell::RefCell;
use std::rc::Rc;

use nalgebra::Vector2;

use crate::body::Body;
use crate::checks::check_collision;
use crate::collision::Collision;
use crate::quad_tree;
use crate::shape::{Circle, Shape, AABB};

/**
 * Sets velocity in m/s
 */
fn resolve_collision(a: &mut Body, b: &mut Body, collision: &Collision) {
    if a.fixed && b.fixed {
        return;
    }
    let relative_velocity = b.velocity - a.velocity;
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

    let impulse_vector = impulse * collision.normal;

    if !a.fixed {
        a.velocity -= a.inv_mass * impulse_vector;
    }
    if !b.fixed {
        b.velocity += b.inv_mass * impulse_vector;
    }
}

fn correct_position(a: &mut Body, b: &mut Body, collision: &Collision) {
    if a.fixed && b.fixed {
        return;
    }
    // High percentage = no penetration
    let percent = 0.75;
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

    let correction = collision.normal * correction_scalar;
    if !a.fixed {
        a.position -= a.inv_mass * correction;
    }
    if !b.fixed {
        b.position += b.inv_mass * correction;
    }
}

pub struct PhysicsWorld {
    bodies: Vec<Rc<RefCell<Body>>>,
    quad_tree: quad_tree::QuadTree,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        return PhysicsWorld {
            bodies: vec![],
            quad_tree: quad_tree::QuadTree::new(0, AABB::new(20f32, 20f32, 500f32, 300f32)),
        };
    }
    pub fn add_body(&mut self, body: Body) -> Rc<RefCell<Body>> {
        let body_ref = Rc::new(RefCell::new(body));
        self.bodies.push(Rc::clone(&body_ref));
        return body_ref;
    }
    pub fn remove_body(&mut self, body: Body) {
        unimplemented!();
    }
    fn calc_velocity(&mut self, dt: f32) {
        // Update position of bodies based on velocity
        for body in self.bodies.iter() {
            let mut body_mut = body.borrow_mut();

            // Apply force in body
            // TODO: Fix force code
            // this is not really using any fancy physics, it's just me
            let linear_acceleration = body_mut.force / body_mut.mass;
            body_mut.velocity = body_mut.velocity + linear_acceleration * dt;
            // Force has been applied, reset it in body
            body_mut.force = Vector2::new(0f32, 0f32);

            // Apply friction based on surface
            // TODO: check surface friction
            let friction_val = (body_mut.friction * body_mut.velocity) * dt;
            body_mut.velocity -= friction_val;

            if body_mut.velocity.abs() < Vector2::new(0.1, 0.1) {
                body_mut.velocity = Vector2::new(0f32, 0f32);
            }

            body_mut.position = body_mut.position + body_mut.velocity * dt;
        }
    }
    pub fn update_with_quad(&mut self, dt: f32) {
        self.calc_velocity(dt);
        self.quad_tree.clear();
        for body in self.bodies.iter() {
            self.quad_tree.insert(Rc::clone(body));
        }
        let collisions = self.quad_tree.check_collisions();
        for collision in collisions {
            resolve_collision(
                &mut collision.a.borrow_mut(),
                &mut collision.b.borrow_mut(),
                &collision,
            );
            correct_position(
                &mut collision.a.borrow_mut(),
                &mut collision.b.borrow_mut(),
                &collision,
            );
        }
    }
    pub fn get_quad_tree_aabb(&self) -> Vec<AABB> {
        return self.quad_tree.get_node_aabb();
    }
    pub fn update(&mut self, dt: f32) {
        self.calc_velocity(dt);
        // Resolve collision for body pairs
        for (i, a) in self.bodies.iter().enumerate() {
            for b in &self.bodies[(i + 1)..] {
                let resolution = check_collision(&a, &b);
                match resolution {
                    Some(collision) => {
                        resolve_collision(&mut a.borrow_mut(), &mut b.borrow_mut(), &collision);
                        correct_position(&mut a.borrow_mut(), &mut b.borrow_mut(), &collision);
                    }
                    None => {}
                }
            }
        }
    }
}
