#![deny(clippy::all)]
#![warn(clippy::pedantic)]

pub mod body;
mod checks;
mod collision;
mod quad_tree;
pub mod shape;
mod vec;
pub mod world;
pub use vec::Vec2;

#[cfg(feature = "bevy")]
pub mod bevy;
