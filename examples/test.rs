extern crate phusis;
extern crate stopwatch;

use phusis::body::Body;
use phusis::shape::{Circle, ShapeKind, AABB};
use phusis::world::PhysicsWorld;

// use ggez::event::{self, EventHandler};
// use ggez::{graphics, graphics::Rect, mouse, timer, Context, ContextBuilder, GameResult};

use bevy::prelude::Vec2;

use rand::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

const NS_PER_UPDATE: f32 = 1_000_000_000_f32 / 60f32;

fn main() {
    // setup ggez
    let ctx = &mut ContextBuilder::new("Phusis test", "Isidor Nygren")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    // graphics::set_background_color(ctx, graphics::Color::new(1.0,0.0,1.0,1.0));
    let mut my_game = GameState::new(ctx).unwrap();

    // Run!
    match event::run(ctx, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct GameState {
    bodies: Vec<Rc<RefCell<Body>>>,
    physics_world: PhysicsWorld,
}

impl GameState {
    fn new(_ctx: &mut Context) -> GameResult<GameState> {
        // Initiate physics simulation
        let mut bodies: Vec<Rc<RefCell<Body>>> = vec![];

        let mut physics_world = PhysicsWorld::new();
        // Mouse body
        let body = physics_world.add_body(Body::new(
            100f32,
            1f32,
            Box::new(AABB::new(0f32, 0f32, 20f32, 20f32)),
            Vec2::new(32f32, 256f32),
            false,
        ));
        bodies.push(body);

        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let x = rng.gen_range(36..724) as f32;
            let y = rng.gen_range(36..524) as f32;

            let new_body = physics_world.add_body(Body::new(
                1f32,
                1f32,
                Box::new(Circle::new(10f32)),
                Vec2::new(x, y),
                false,
            ));
            bodies.push(new_body);
        }
        let s = GameState {
            bodies,
            physics_world,
        };
        Ok(s)
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        // Update first body to mouse position
        let mouse_pos = mouse::get_position(ctx).unwrap();
        let mouse_vec = Vec2::new(mouse_pos.x, mouse_pos.y);
        let vec_diff = mouse_vec - self.bodies[0].borrow().position;
        self.bodies[0].borrow_mut().velocity = vec_diff;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);
            self.physics_world.update_with_quad(seconds);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        // draw the quad tree aabb
        /* let qq_aabb = self.physics_world.get_quad_tree_aabb();
        for aabb in qq_aabb {
            let body_rect = aabb.get_rect();
            graphics::set_color(ctx, graphics::Color::new(0.0, 1.0, 0.0, 1.0)).unwrap();
            graphics::rectangle(
                ctx,
                graphics::DrawMode::Line(1f32),
                Rect::new(body_rect.0, body_rect.1, body_rect.2, body_rect.3),
            )
            .unwrap();
        } */

        // canvas.set_draw_color(Color::RGB(255, 255, 255));
        for body in self.bodies.clone() {
            let borrowed_body = body.borrow();
            // draw aabb of body
            /*let body_rect = borrowed_body.get_aabb().get_rect();
            graphics::set_color(ctx, graphics::Color::new(1.0, 0.0, 0.0, 1.0)).unwrap();
            graphics::rectangle(
                ctx,
                graphics::DrawMode::Line(1f32),
                Rect::new(body_rect.0, body_rect.1, body_rect.2, body_rect.3),
            )
            .unwrap();*/
            match borrowed_body.shape.get_kind() {
                ShapeKind::Circle => {
                    graphics::set_color(ctx, graphics::Color::new(0.0, 1.0, 1.0, 1.0)).unwrap();
                    graphics::circle(
                        ctx,
                        graphics::DrawMode::Line(2f32),
                        graphics::Point2::new(borrowed_body.position.x, borrowed_body.position.y),
                        borrowed_body.shape.get_radius(),
                        0.5f32,
                    )
                    .unwrap();
                }
                ShapeKind::AABB => {
                    graphics::set_color(ctx, graphics::Color::new(0.0, 1.0, 0.0, 1.0)).unwrap();
                    let body_rect = borrowed_body.get_aabb().get_rect();
                    graphics::rectangle(
                        ctx,
                        graphics::DrawMode::Line(1f32),
                        Rect::new(
                            body_rect.0 - body_rect.2 / 2f32,
                            body_rect.1 - body_rect.3 / 2f32,
                            body_rect.2,
                            body_rect.3,
                        ),
                    )
                    .unwrap();
                }
            }
        }
        graphics::present(ctx);
        Ok(())
    }
}
