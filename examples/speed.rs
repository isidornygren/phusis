extern crate phusis;
extern crate stopwatch;

use phusis::shape::Circle;
use phusis::world::PhysicsWorld;
use phusis::{body::Body, Vec2};

use rand::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

const S_PER_UPDATE: f32 = 1f32 / 60f32;

const ITERATIONS: u32 = 100;
const UPDATES: u32 = 1000;
const BODIES: u32 = 200;

fn main() {
    let mut rng = rand::thread_rng();

    println!("------ Testing ------ \n");
    println!("Testing with quad tree");
    let mut tot_us = 0;
    for _ in 0..ITERATIONS {
        let mut physics_world = PhysicsWorld::new();
        for _ in 0..BODIES {
            let x = rng.gen_range(36..724) as f32;
            let y = rng.gen_range(36..524) as f32;

            let new_body = physics_world.add_body(Body::new(
                1f32,
                1f32,
                Box::new(Circle::new(8f32)),
                Vec2::new(x, y),
                false,
            ));
        }
        for _ in 0..UPDATES {
            let now = Instant::now();
            physics_world.update_with_quad(S_PER_UPDATE);
            let us_since_now = now.elapsed().as_micros();
            tot_us += us_since_now;
        }
    }
    println!(
        "Finished testing quad tree after {} iterations & {} updates with {} bodies,\n results:",
        ITERATIONS, UPDATES, BODIES
    );
    println!("====================================");
    println!(
        " Avg:          {} μs",
        tot_us / (ITERATIONS * UPDATES) as u128
    );
    println!(
        " Avg per body: {} μs",
        tot_us / (ITERATIONS * UPDATES * BODIES) as u128
    );
    println!(" Total:        {} μs", tot_us);
    println!("====================================");
}
