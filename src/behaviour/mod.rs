use bevy::{
    prelude::{IntoSystemDescriptor, SystemSet},
    time::FixedTimestep,
};

use crate::config;

pub mod eating;
pub mod growth;
pub mod lifecycle;
pub mod metabolism;
pub mod movement;
pub mod sight;
pub mod thinking;
pub mod timers;

pub fn time_step_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(config::TIME_STEP as f64))
        .with_system(timers::progress_age_system.before(thinking::thinking_system))
        .with_system(timers::progress_timers_system.before(thinking::thinking_system))
        .with_system(thinking::sensory_system.before(thinking::thinking_system))
        .with_system(sight::process_sight_system.before(thinking::thinking_system))
        .with_system(thinking::thinking_system)
        .with_system(timers::reset_internal_timer_system.after(thinking::thinking_system))
        .with_system(movement::movement_system.after(thinking::thinking_system))
        .with_system(eating::process_eaters_system.after(thinking::thinking_system))
        .with_system(lifecycle::process_layers_system.after(thinking::thinking_system))
        .with_system(growth::process_growers_system.after(thinking::thinking_system))
        .with_system(lifecycle::transition_to_adult_system)
        .with_system(lifecycle::transition_to_hatching_system)
        .with_system(eating::attempted_to_eat_system.after(eating::process_eaters_system))
        .with_system(growth::attempted_to_grow_system.after(growth::process_growers_system))
        .with_system(growth::existence_system)
}

pub fn egg_spawning_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(1.0))
        .with_system(lifecycle::spawn_egg_system)
}

pub fn slow_behaviour_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(0.1))
        .with_system(lifecycle::lay_egg_system.after(lifecycle::process_layers_system))
        .with_system(growth::grow_bug_system.after(growth::attempted_to_grow_system))
}

pub fn metabolism_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(1.0))
        .with_system(metabolism::energy_return_system)
}

pub fn despawn_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(0.1))
        .with_system(lifecycle::kill_bug_system)
        .with_system(lifecycle::hatch_egg_system)
}

pub fn eating_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(0.1))
        .with_system(eating::eating_system.after(eating::attempted_to_eat_system))
}
