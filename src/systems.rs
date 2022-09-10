use bevy::{prelude::*, time::FixedTimestep};

use crate::{body, config, ecosystem, interaction, mind, movement, spawn, ui};

pub fn interaction_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(0.15))
        .with_system(interaction::move_camera_system)
        .with_system(ui::select_bug_system)
        .with_system(ui::selected_bug_system)
}

pub fn behavior_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(config::TIME_STEP as f64))
        .with_system(body::progress_age_system.before(mind::thinking_system))
        .with_system(mind::thinking_system)
        .with_system(mind::sensory_system.before(mind::thinking_system))
        .with_system(movement::movement_system.after(mind::thinking_system))
        .with_system(mind::process_eaters_system.after(mind::thinking_system))
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

pub fn slow_behavior_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(0.1))
        .with_system(mind::eating_system.after(mind::process_eaters_system))
        .with_system(mind::attempted_to_eat_system.after(mind::eating_system))
}

pub fn burnt_energy_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(1.0))
        .with_system(ecosystem::burnt_energy_system)
        .with_system(mind::thinking_energy_system)
}
