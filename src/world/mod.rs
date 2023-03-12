use generational_arena::{Arena, Index};

use self::broad::{BroadPhase, BroadPhaseElement};
use crate::{
    body::{Body, BodySensor},
    checks::{circle_vs_circle, rect_vs_circle, rect_vs_rect},
    collision::Collision,
    shape::Shape,
    Vec2,
};

pub mod broad;

// High percentage = no penetration
const PENETRATION_PERCENTAGE: f32 = 0.5;
// Allows penetration without jittering
const K_SLOP: f32 = 0.01;

/**
 * Sets velocity in m/s
 */
fn resolve_collision(bodies: &mut Arena<Body>, collision: &Collision<f32, ArenaHandle>) {
    let (a_fixed, b_fixed, a_inv_mass, b_inv_mass, impulse_vector) = {
        let a = bodies.get(collision.a).unwrap();
        let b = bodies.get(collision.b).unwrap();

        if a.fixed && b.fixed {
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
        bodies.get_mut(collision.a).unwrap().velocity -= impulse_vector * a_inv_mass;
    }
    if !b_fixed {
        bodies.get_mut(collision.b).unwrap().velocity += impulse_vector * b_inv_mass;
    }
}

fn correct_position(bodies: &mut Arena<Body>, collision: &Collision<f32, ArenaHandle>) {
    let (a_fixed, b_fixed, a_inv_mass, b_inv_mass, correction) = {
        let a = bodies.get(collision.a).unwrap();
        let b = bodies.get(collision.b).unwrap();

        if a.fixed && b.fixed {
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
        bodies.get_mut(collision.a).unwrap().position -= correction * a_inv_mass;
    }
    if !b_fixed {
        bodies.get_mut(collision.b).unwrap().position += correction * b_inv_mass;
    }
}

#[derive(Debug, Clone)]
pub struct SensorHandle {
    pub index: usize,
}

pub type ArenaHandle = Index;

pub struct PhysicsWorld<Broad>
where
    Broad: BroadPhase<ArenaHandle>, {
    bodies:          Arena<Body>,
    sensors:         Vec<BodySensor>,
    pub broad_phase: Broad,
}

impl<Broad> PhysicsWorld<Broad>
where
    Broad: BroadPhase<ArenaHandle>,
{
    pub fn new(broad_phase: Broad) -> Self {
        Self {
            bodies: Arena::new(),
            sensors: vec![],
            broad_phase,
        }
    }

    pub fn add_sensor(&mut self, sensor: BodySensor) -> SensorHandle {
        self.sensors.push(sensor);
        SensorHandle {
            index: self.bodies.len() - 1,
        }
    }

    pub fn add_body(&mut self, body: Body) -> ArenaHandle {
        let aabb = body.get_aabb();
        let handle = self.bodies.insert(body);
        self.broad_phase.insert(BroadPhaseElement { aabb, handle });

        handle
    }

    pub fn remove_body(&mut self, handle: &ArenaHandle) {
        let body = self.bodies.remove(*handle).unwrap();
        self.broad_phase.remove(BroadPhaseElement {
            handle: *handle,
            aabb:   body.get_aabb(),
        });
    }

    pub fn update<F>(&mut self, handle: &ArenaHandle, mut func: F)
    where
        F: FnMut(&mut Body), {
        let body = self.bodies.get_mut(*handle).unwrap();
        self.broad_phase.remove(BroadPhaseElement {
            aabb:   body.get_aabb(),
            handle: *handle,
        });
        func(body);
        self.broad_phase.insert(BroadPhaseElement {
            aabb:   body.get_aabb(),
            handle: *handle,
        });
    }

    #[must_use]
    pub fn get_body(&self, handle: &ArenaHandle) -> Option<&Body> {
        self.bodies.get(*handle)
    }

    #[must_use]
    pub fn get_body_mut(&mut self, handle: &ArenaHandle) -> Option<&mut Body> {
        self.bodies.get_mut(*handle)
    }

    fn calc_velocity(&mut self, dt: f32) {
        // Update position of bodies based on velocity
        for (_, body) in &mut self.bodies {
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

    pub fn update_with_quad(&mut self, dt: f32) -> Vec<Collision<f32, ArenaHandle>> {
        self.calc_velocity(dt);
        // Broad phase
        let broad_collisions = self.broad_phase.check_collisions();

        // for sensor in self.sensors {
        //     // Get sensor collision broad phase
        //     let sensor_broad_collisions = self
        //         .broad_phase
        //         .check(sensor.shape.get_aabb(sensor.position));
        //     // Get sensor collision narrow phase
        //     sensor_broad_collisions
        //         .iter()
        //         .fold(vec![], |mut narrow_collisions, collision| {
        //             // let a_body = sensorrap();
        //             let b_body = self.bodies.get(collision.index).unwrap();

        //             let maybe_contact = match (&sensor.shape, &b_body.shape) {
        //                 (Shape::Circle(a_circle), Shape::Circle(b_circle)) => {
        //                     circle_vs_circle(a_circle, b_circle, &sensor.position, &b_body.position)
        //                 },
        //                 (Shape::Rect(a_rect), Shape::Rect(b_rect)) => {
        //                     rect_vs_rect(a_rect, b_rect, &sensor.position, &b_body.position)
        //                 },
        //                 (Shape::Circle(circle), Shape::Rect(rect))
        //                 | (Shape::Rect(rect), Shape::Circle(circle)) => {
        //                     rect_vs_circle(rect, circle, &sensor.position, &b_body.position)
        //                 },
        //             };

        //             if let Some(contact) = maybe_contact {
        //                 narrow_collisions.push(Collision {
        //                     a: collision.a.clone(),
        //                     b: collision.clone(),
        //                     contact,
        //                 });
        //             }
        //             narrow_collisions
        //         });
        // }

        // Narrow phase
        let collisions =
            broad_collisions
                .iter()
                .fold(vec![], |mut narrow_collisions, collision| {
                    let a_body = self.bodies.get(collision.a).unwrap();
                    let b_body = self.bodies.get(collision.b).unwrap();

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
                            a: collision.a,
                            b: collision.b,
                            contact,
                        });
                    }
                    narrow_collisions
                });

        for collision in &collisions {
            let a_sensor_or_fixed = self
                .get_body(&collision.a)
                .map_or(false, |a_body| a_body.fixed);
            let b_sensor_or_fixed = self
                .get_body(&collision.b)
                .map_or(false, |b_body| b_body.fixed);

            if !a_sensor_or_fixed {
                self.broad_phase.remove(BroadPhaseElement {
                    aabb:   self.bodies.get(collision.a).unwrap().get_aabb(),
                    handle: collision.a,
                });
            }
            if !b_sensor_or_fixed {
                self.broad_phase.remove(BroadPhaseElement {
                    aabb:   self.bodies.get(collision.b).unwrap().get_aabb(),
                    handle: collision.b,
                });
            }

            resolve_collision(&mut self.bodies, collision);
            correct_position(&mut self.bodies, collision);

            if !a_sensor_or_fixed {
                self.broad_phase.insert(BroadPhaseElement {
                    handle: collision.a,
                    aabb:   self.get_body(&collision.a).unwrap().get_aabb(),
                });
            }
            if !b_sensor_or_fixed {
                self.broad_phase.insert(BroadPhaseElement {
                    handle: collision.b,
                    aabb:   self.get_body(&collision.b).unwrap().get_aabb(),
                });
            }
        }
        self.broad_phase.clean_up();

        collisions
    }
}
