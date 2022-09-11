#![allow(dead_code)]
mod attributes;
mod body;
pub mod config;
mod ecosystem;
mod food;
mod interaction;
mod mind;
mod movement;
pub mod setup;
mod spawn;
pub mod systems;
pub mod ui;

pub mod resources {
    pub use crate::ecosystem::Ecosystem;
}
