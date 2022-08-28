use bevy::{prelude::*, time::FixedTimestep};

use crate::{config, interaction, mind, movement};

pub fn moving_camera_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(0.15))
        .with_system(interaction::move_camera_system)
}

pub fn thinking_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(config::TIME_STEP as f64))
        .with_system(mind::thinking_system)
}

pub fn movement_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(config::TIME_STEP as f64))
        .with_system(movement::movement_system)
}
