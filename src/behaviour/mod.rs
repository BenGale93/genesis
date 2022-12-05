use std::time::Duration;

use bevy::prelude::{App, Plugin, SystemSet};
use iyes_loopless::prelude::*;

use crate::ui;

pub mod eating;
pub mod growth;
pub mod lifecycle;
pub mod metabolism;
pub mod movement;
pub mod sight;
pub mod thinking;
pub mod timers;

pub fn before_thinking_system_set() -> SystemSet {
    ConditionSet::new()
        .label("before_thinking")
        .before("thinking")
        .run_if_not(ui::is_paused)
        .with_system(timers::progress_age_system)
        .with_system(timers::progress_timers_system)
        .with_system(thinking::sensory_system)
        .with_system(sight::process_sight_system)
        .into()
}

pub fn thinking_system_set() -> SystemSet {
    ConditionSet::new()
        .label("thinking")
        .run_if_not(ui::is_paused)
        .with_system(thinking::thinking_system)
        .into()
}

pub fn after_thinking_system_set() -> SystemSet {
    ConditionSet::new()
        .label("after_thinking")
        .after("thinking")
        .run_if_not(ui::is_paused)
        .with_system(timers::reset_internal_timer_system)
        .with_system(movement::movement_system)
        .with_system(eating::process_eaters_system)
        .with_system(lifecycle::process_layers_system)
        .with_system(growth::process_growers_system)
        .into()
}

pub fn attempting_behaviour_system_set() -> SystemSet {
    ConditionSet::new()
        .label("attempting")
        .after("after_thinking")
        .run_if_not(ui::is_paused)
        .with_system(eating::attempted_to_eat_system)
        .with_system(growth::attempted_to_grow_system)
        .into()
}

pub fn other_behaviour_system_set() -> SystemSet {
    ConditionSet::new()
        .label("other")
        .run_if_not(ui::is_paused)
        .with_system(lifecycle::transition_to_adult_system)
        .with_system(lifecycle::transition_to_hatching_system)
        .with_system(growth::existence_system)
        .with_system(timers::progress_simulation_timer)
        .with_system(eating::eating_system)
        .into()
}

pub fn slow_behaviour_system_set() -> SystemSet {
    ConditionSet::new()
        .run_if_not(ui::is_paused)
        .with_system(lifecycle::lay_egg_system)
        .with_system(growth::grow_bug_system)
        .into()
}

pub fn very_slow_system_set() -> SystemSet {
    ConditionSet::new()
        .run_if_not(ui::is_paused)
        .with_system(metabolism::energy_return_system)
        .with_system(lifecycle::spawn_egg_system)
        .into()
}

pub struct GenesisBehaviourPlugin;

impl Plugin for GenesisBehaviourPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(timers::SimulationTime::default())
            .add_fixed_timestep(Duration::from_millis(100), "slow")
            .add_fixed_timestep(Duration::from_secs(1), "very_slow")
            .add_fixed_timestep_system_set("very_slow", 0, very_slow_system_set())
            .add_fixed_timestep_system_set("slow", 0, slow_behaviour_system_set())
            .add_fixed_timestep(Duration::from_secs_f32(1.0 / 60.0), "standard")
            .add_fixed_timestep_system_set("standard", 0, before_thinking_system_set())
            .add_fixed_timestep_system_set("standard", 0, thinking_system_set())
            .add_fixed_timestep_system_set("standard", 0, after_thinking_system_set())
            .add_fixed_timestep_system_set("standard", 0, attempting_behaviour_system_set())
            .add_fixed_timestep_system_set("standard", 0, other_behaviour_system_set());
    }
}
