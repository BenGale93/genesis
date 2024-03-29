use bevy::prelude::{App, Plugin, SystemSet};
use genesis_components as components;
use genesis_config::{BEHAVIOUR_TICK, SLOW_BEHAVIOUR_TICK, VERY_SLOW_BEHAVIOUR_TICK};
use iyes_loopless::prelude::*;

use crate::{conditions, SimState};

pub mod attacking;
pub mod eating;
pub mod grabbing;
pub mod growing;
pub mod laying;
pub mod metabolism;
pub mod moving;
pub mod seeing;
pub mod thinking;
pub mod timing;

pub fn before_thinking_system_set() -> SystemSet {
    ConditionSet::new()
        .label("before_thinking")
        .before("thinking")
        .run_in_state(SimState::Simulation)
        .run_if_not(conditions::is_paused)
        .with_system(thinking::sensory_system)
        .with_system(seeing::process_sight_system)
        .with_system(metabolism::update_health_efficiency_system)
        .into()
}

pub fn thinking_system_set() -> SystemSet {
    ConditionSet::new()
        .label("thinking")
        .run_in_state(SimState::Simulation)
        .run_if_not(conditions::is_paused)
        .with_system(thinking::thinking_system)
        .into()
}

pub fn after_thinking_system_set() -> SystemSet {
    ConditionSet::new()
        .label("after_thinking")
        .after("thinking")
        .run_in_state(SimState::Simulation)
        .run_if_not(conditions::is_paused)
        .with_system(timing::reset_internal_timer_system)
        .with_system(moving::movement_system)
        .with_system(eating::process_eaters_system)
        .with_system(laying::process_layers_system)
        .with_system(growing::process_growers_system)
        .with_system(grabbing::process_grabbers_system)
        .with_system(eating::digestion_intensity_system)
        .into()
}

pub fn attempting_behaviour_system_set() -> SystemSet {
    ConditionSet::new()
        .label("attempting")
        .after("after_thinking")
        .run_in_state(SimState::Simulation)
        .run_if_not(conditions::is_paused)
        .with_system(eating::attempted_to_eat_system)
        .with_system(laying::attempted_to_lay_system)
        .with_system(growing::attempted_to_grow_system)
        .with_system(grabbing::attempted_to_grab_system)
        .into()
}

pub fn other_behaviour_system_set() -> SystemSet {
    ConditionSet::new()
        .label("other")
        .run_if_not(conditions::is_paused)
        .run_in_state(SimState::Simulation)
        .with_system(growing::existence_system)
        .with_system(eating::eating_system)
        .with_system(grabbing::grabbing_system)
        .with_system(attacking::attack_bug_system)
        .with_system(attacking::attack_egg_system)
        .into()
}

pub fn slow_behaviour_system_set() -> SystemSet {
    ConditionSet::new()
        .run_if_not(conditions::is_paused)
        .run_in_state(SimState::Simulation)
        .with_system(laying::lay_egg_system)
        .with_system(growing::grow_bug_system)
        .with_system(metabolism::heal_damage_system)
        .into()
}

pub fn very_slow_system_set() -> SystemSet {
    ConditionSet::new()
        .run_if_not(conditions::is_paused)
        .run_in_state(SimState::Simulation)
        .with_system(metabolism::energy_return_system)
        .with_system(laying::spawn_egg_system)
        .with_system(eating::digest_food_system)
        .into()
}

pub fn timers_system_set() -> SystemSet {
    ConditionSet::new()
        .run_in_state(SimState::Simulation)
        .run_if_not(conditions::is_paused)
        .with_system(timing::progress_simulation_timer)
        .with_system(timing::progress_age_system)
        .with_system(timing::progress_timers_system)
        .into()
}

pub struct GenesisBehaviourPlugin;

impl Plugin for GenesisBehaviourPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(components::ComponentsPlugin)
            .add_fixed_timestep(VERY_SLOW_BEHAVIOUR_TICK, "very_slow")
            .add_fixed_timestep(SLOW_BEHAVIOUR_TICK, "slow")
            .add_fixed_timestep(BEHAVIOUR_TICK, "standard")
            .add_fixed_timestep_system_set("very_slow", 0, very_slow_system_set())
            .add_fixed_timestep_system_set("slow", 0, slow_behaviour_system_set())
            .add_fixed_timestep_system_set("standard", 0, before_thinking_system_set())
            .add_fixed_timestep_system_set("standard", 0, thinking_system_set())
            .add_fixed_timestep_system_set("standard", 0, after_thinking_system_set())
            .add_fixed_timestep_system_set("standard", 0, attempting_behaviour_system_set())
            .add_fixed_timestep_system_set("standard", 0, other_behaviour_system_set())
            .add_system_set(timers_system_set());
    }
}
