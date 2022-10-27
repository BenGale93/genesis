use bevy::{
    prelude::{ParallelSystemDescriptorCoercion, SystemSet},
    time::FixedTimestep,
};

use crate::config;

pub mod eating;
pub mod lifecycle;
pub mod metabolism;
pub mod movement;
pub mod sight;
pub mod thinking;
pub mod timers;

pub fn behaviour_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(config::TIME_STEP as f64))
        .with_system(timers::progress_age_system.before(thinking::thinking_system))
        .with_system(timers::progress_timers_system.before(thinking::thinking_system))
        .with_system(thinking::sensory_system.before(thinking::thinking_system))
        .with_system(thinking::thinking_system)
        .with_system(timers::reset_internal_timer_system.after(thinking::thinking_system))
        .with_system(movement::movement_system.after(thinking::thinking_system))
        .with_system(eating::process_eaters_system.after(thinking::thinking_system))
        .with_system(lifecycle::process_layers_system.after(thinking::thinking_system))
        .with_system(sight::process_sight_system)
        .with_system(lifecycle::transition_to_adult_system)
        .with_system(lifecycle::transition_to_hatching_system)
        .with_system(lifecycle::kill_bug_system)
}

pub fn egg_spawning_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(1.0))
        .with_system(lifecycle::spawn_egg_system)
}

pub fn plant_spawning_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(config::TIME_STEP as f64))
        .with_system(lifecycle::spawn_plant_system)
}

pub fn slow_behaviour_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(0.1))
        .with_system(eating::eating_system.after(eating::process_eaters_system))
        .with_system(lifecycle::lay_egg_system.after(lifecycle::process_layers_system))
        .with_system(metabolism::attempted_to_eat_system.after(eating::eating_system))
        .with_system(lifecycle::hatch_egg_system)
}

pub fn metabolism_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(1.0))
        .with_system(metabolism::burnt_energy_system)
        .with_system(metabolism::thinking_energy_system)
        .with_system(metabolism::movement_energy_burn_system)
}
