use bevy::{prelude::*, time::FixedTimestep};

use crate::{config, interaction, mind, movement, spawn};

pub fn moving_camera_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(0.15))
        .with_system(interaction::move_camera_system)
}

pub fn behavior_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(config::TIME_STEP as f64))
        .with_system(mind::thinking_system)
        .with_system(mind::sensory_system.before(mind::thinking_system))
        .with_system(movement::movement_system.after(mind::thinking_system))
}

pub fn bug_spawning_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(1.0))
        .with_system(spawn::spawn_bug_system)
}

pub fn food_spawning_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(config::TIME_STEP as f64))
        .with_system(spawn::spawn_food_system)
}
