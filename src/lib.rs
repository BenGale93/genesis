#![allow(dead_code)]
mod attributes;
mod body;
pub mod config;
mod ecosystem;
mod interaction;
mod mind;
mod movement;
pub mod setup;
mod sight;
mod spawn;
pub mod systems;
pub mod ui;

pub mod resources {
    pub use crate::ecosystem::Ecosystem;
}
