use std::time::Duration;

use bevy::prelude::{App, IntoSystemDescriptor, Plugin, SystemSet};
use iyes_loopless::prelude::*;

pub mod eating;
pub mod growth;
pub mod lifecycle;
pub mod metabolism;
pub mod movement;
pub mod sight;
pub mod thinking;
pub mod timers;

pub fn behaviour_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(timers::progress_age_system.before("thinking"))
        .with_system(timers::progress_timers_system.before("thinking"))
        .with_system(thinking::sensory_system.before("thinking"))
        .with_system(sight::process_sight_system.before("thinking"))
        .with_system(thinking::thinking_system.label("thinking"))
        .with_system(timers::reset_internal_timer_system.after("thinking"))
        .with_system(movement::movement_system.after("thinking"))
        .with_system(
            eating::process_eaters_system
                .label("process_eaters")
                .after("thinking"),
        )
        .with_system(
            lifecycle::process_layers_system
                .label("process_layers")
                .after("thinking"),
        )
        .with_system(
            growth::process_growers_system
                .label("process_growers")
                .after("thinking"),
        )
        .with_system(lifecycle::transition_to_adult_system)
        .with_system(lifecycle::transition_to_hatching_system)
        .with_system(eating::attempted_to_eat_system.after("process_eaters"))
        .with_system(
            growth::attempted_to_grow_system
                .label("attempted_to_grow")
                .after("process_growers"),
        )
        .with_system(growth::existence_system)
}

pub fn slow_behaviour_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(lifecycle::lay_egg_system)
        .with_system(growth::grow_bug_system)
        .with_system(eating::eating_system)
}

pub struct GenesisBehaviourPlugin;

impl Plugin for GenesisBehaviourPlugin {
    fn build(&self, app: &mut App) {
        app.add_fixed_timestep(Duration::from_millis(100), "slow")
            .add_fixed_timestep(Duration::from_secs(1), "very_slow")
            .add_fixed_timestep_system("very_slow", 0, metabolism::energy_return_system)
            .add_fixed_timestep_system("very_slow", 0, lifecycle::spawn_egg_system)
            .add_fixed_timestep_system_set("slow", 0, slow_behaviour_system_set())
            .add_system_set(behaviour_system_set());
    }
}
