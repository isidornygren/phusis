use crate::{
    body::Body,
    checks::{circle_vs_circle, rect_vs_circle, rect_vs_rect},
    collision::{Collision, Contact},
    quad_tree::{QuadElement, QuadTree},
    shape::{Shape, AABB},
    Vec2,
};

// High percentage = no penetration
const PENETRATION_PERCENTAGE: f32 = 0.5;
// Allows penetration without jittering
const K_SLOP: f32 = 0.01;

/**
 * Sets velocity in m/s
 */
fn resolve_collision(bodies: &mut [Body], collision: &Collision<f32>) {
    let (a_fixed, b_fixed, a_inv_mass, b_inv_mass, impulse_vector) = {
        let a = bodies.get(collision.a.index).unwrap();
        let b = bodies.get(collision.b.index).unwrap();

        if (a.fixed && b.fixed) || a.sensor || b.sensor {
            return;
        }
        let relative_velocity = b.velocity - a.velocity;
        let velocity_along_normal = relative_velocity.dot(&collision.contact.normal);

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

        let impulse_vector = collision.contact.normal * impulse;

        (a.fixed, b.fixed, a.inv_mass, b.inv_mass, impulse_vector)
    };

    if !a_fixed {
        bodies.get_mut(collision.a.index).unwrap().velocity -= impulse_vector * a_inv_mass;
    }
    if !b_fixed {
        bodies.get_mut(collision.b.index).unwrap().velocity += impulse_vector * b_inv_mass;
    }
}

fn correct_position(bodies: &mut [Body], collision: &Collision<f32>) {
    // println!("Correcting position: {:?}", collision);
    let (a_fixed, b_fixed, a_inv_mass, b_inv_mass, correction) = {
        let a = bodies.get(collision.a.index).unwrap();
        let b = bodies.get(collision.b.index).unwrap();

        if a.fixed && b.fixed || a.sensor || b.sensor {
            return;
        }

        let maximum = 0f32.max(collision.contact.penetration_depth - K_SLOP);

        let correction_scalar = if b.fixed {
            maximum / a.inv_mass * PENETRATION_PERCENTAGE
        } else if a.fixed {
            maximum / b.inv_mass * PENETRATION_PERCENTAGE
        } else {
            maximum / (a.inv_mass + b.inv_mass) * PENETRATION_PERCENTAGE
        };

        let correction = collision.contact.normal * correction_scalar;
        (a.fixed, b.fixed, a.inv_mass, b.inv_mass, correction)
    };
    if !a_fixed {
        bodies.get_mut(collision.a.index).unwrap().position -= correction * a_inv_mass;
    }
    if !b_fixed {
        bodies.get_mut(collision.b.index).unwrap().position += correction * b_inv_mass;
    }
}

#[derive(Debug, Clone)]
pub struct BodyHandle {
    pub index: usize,
}

pub struct PhysicsWorld {
    pub bodies:      Vec<Body>,
    removed_indices: Vec<usize>,
    pub quad_tree:   QuadTree,
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self {
            bodies:          vec![],
            quad_tree:       QuadTree::new(0, AABB::new(-1000, -1000, 1000, 1000)),
            removed_indices: vec![],
        }
    }
}

impl PhysicsWorld {
    pub fn add_body(&mut self, body: Body) -> BodyHandle {
        if let Some(removed_index) = self.removed_indices.pop() {
            let handle = BodyHandle {
                index: removed_index,
            };

            self.quad_tree.insert(QuadElement {
                handle: handle.clone(),
                aabb:   body.get_aabb(),
            });

            *self.bodies.get_mut(removed_index).unwrap() = body;

            return handle;
        }
        let handle = BodyHandle {
            index: self.bodies.len(),
        };
        self.quad_tree.insert(QuadElement {
            handle: handle.clone(),
            aabb:   body.get_aabb(),
        });
        self.bodies.push(body);
        handle
    }

    pub fn remove_body(&mut self, handle: BodyHandle) {
        self.removed_indices.push(handle.index);
        self.quad_tree.remove(QuadElement {
            handle: handle.clone(),
            aabb:   self.bodies.get(handle.index).unwrap().get_aabb(),
        });
    }

    pub fn remove_from_quad_tree(&mut self, handle: &BodyHandle) {
        self.quad_tree.remove(QuadElement {
            handle: handle.clone(),
            aabb:   self.bodies.get(handle.index).unwrap().get_aabb(),
        });
    }

    pub fn insert_into_quad_tree(&mut self, element: QuadElement) {
        self.quad_tree.insert(element);
    }

    #[must_use]
    pub fn get_body(&self, handle: &BodyHandle) -> Option<&Body> {
        self.bodies.get(handle.index)
    }

    pub fn get_body_mut(&mut self, handle: &BodyHandle) -> Option<&mut Body> {
        self.bodies.get_mut(handle.index)
    }

    fn calc_velocity(&mut self, dt: f32) {
        // Update position of bodies based on velocity
        for body in &mut self.bodies {
            // TODO: Fix force code
            // this is not really using any fancy physics, it's just me (???!!!)
            let linear_acceleration = body.force / body.mass;
            body.velocity += linear_acceleration * dt;
            // Force has been applied, reset it in body
            body.force = Vec2::new(0f32, 0f32);

            // Apply friction based on surface
            let friction_val = body.velocity * body.friction * dt;
            body.velocity -= friction_val;

            if body.velocity.abs() < Vec2::new(0.1, 0.1) {
                body.velocity = Vec2::new(0f32, 0f32);
            }

            body.position += body.velocity * dt;
        }
    }

    pub fn update_with_quad(&mut self, dt: f32) -> Vec<Collision<f32>> {
        self.calc_velocity(dt);
        // Broad phase
        let broad_collisions = self.quad_tree.check_collisions();

        println!("Broad collisions: {:?}", broad_collisions.len());
        // Narrow phase
        let collisions =
            broad_collisions
                .iter()
                .fold(vec![], |mut narrow_collisions, collision| {
                    let a_body = self.bodies.get(collision.a.index).unwrap();
                    let b_body = self.bodies.get(collision.b.index).unwrap();

                    let maybe_contact = match (&a_body.shape, &b_body.shape) {
                        (Shape::Circle(a_circle), Shape::Circle(b_circle)) => {
                            circle_vs_circle(a_circle, b_circle, &a_body.position, &b_body.position)
                        },
                        (Shape::Rect(a_rect), Shape::Rect(b_rect)) => {
                            rect_vs_rect(a_rect, b_rect, &a_body.position, &b_body.position)
                        },
                        (Shape::Circle(circle), Shape::Rect(rect))
                        | (Shape::Rect(rect), Shape::Circle(circle)) => {
                            rect_vs_circle(rect, circle, &a_body.position, &b_body.position)
                        },
                    };

                    if let Some(contact) = maybe_contact {
                        narrow_collisions.push(Collision {
                            a: collision.a.clone(),
                            b: collision.b.clone(),
                            contact,
                        });
                    }
                    narrow_collisions
                });

        for collision in &collisions {
            let a_sensor_or_fixed = self
                .get_body(&collision.a)
                .map_or(false, |a_body| a_body.fixed || a_body.sensor);
            let b_sensor_or_fixed = self
                .get_body(&collision.b)
                .map_or(false, |b_body| b_body.fixed || b_body.sensor);

            if !a_sensor_or_fixed {
                self.quad_tree.remove(QuadElement {
                    handle: collision.a.clone(),
                    aabb:   self.bodies.get(collision.a.index).unwrap().get_aabb(),
                });
            }
            if !b_sensor_or_fixed {
                self.quad_tree.remove(QuadElement {
                    handle: collision.b.clone(),
                    aabb:   self.bodies.get(collision.b.index).unwrap().get_aabb(),
                });
            }

            resolve_collision(&mut self.bodies, collision);
            correct_position(&mut self.bodies, collision);

            if !a_sensor_or_fixed {
                self.quad_tree.insert(QuadElement {
                    handle: collision.a.clone(),
                    aabb:   self.get_body(&collision.a).unwrap().get_aabb(),
                });
            }
            if !b_sensor_or_fixed {
                self.quad_tree.insert(QuadElement {
                    handle: collision.b.clone(),
                    aabb:   self.get_body(&collision.b).unwrap().get_aabb(),
                });
            }
        }

        collisions
    }

    #[must_use]
    pub fn get_quad_tree_aabb(&self) -> Vec<AABB<i32>> {
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
