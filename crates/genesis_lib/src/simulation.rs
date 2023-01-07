use std::time::Duration;

use bevy::prelude::{App, CoreStage, Plugin, Resource, StageLabel, SystemSet, SystemStage};
use bevy_rapier2d::prelude::{ActiveEvents, ColliderMassProperties};
use genesis_attributes as attributes;
use genesis_config as config;
use iyes_loopless::prelude::*;

use crate::{
    behaviour, conditions, genesis_serde, lifecycle, setup, spawning, statistics, ui, SimState,
};

#[derive(Resource, Debug)]
pub struct SimulationSpeed {
    pub speed: f32,
    pub paused: bool,
}

impl Default for SimulationSpeed {
    fn default() -> Self {
        Self {
            speed: 1.0,
            paused: false,
        }
    }
}

pub fn plant_system_set() -> SystemSet {
    ConditionSet::new()
        .run_if_not(conditions::is_paused)
        .run_in_state(SimState::Simulation)
        .with_system(spawning::spawn_plant_system)
        .with_system(spawning::update_plant_size)
        .into()
}

pub fn despawn_system_set() -> SystemSet {
    ConditionSet::new()
        .run_if_not(conditions::is_paused)
        .run_in_state(SimState::Simulation)
        .with_system(lifecycle::kill_bug_system)
        .with_system(behaviour::laying::hatch_egg_system)
        .with_system(spawning::despawn_plants_system)
        .into()
}

pub fn lifecycle_system_set() -> SystemSet {
    ConditionSet::new()
        .label("lifecycle")
        .run_if_not(conditions::is_paused)
        .run_in_state(SimState::Simulation)
        .with_system(lifecycle::transition_to_adult_system)
        .with_system(lifecycle::transition_to_hatching_system)
        .into()
}

pub fn nearest_spawner_system_set() -> SystemSet {
    ConditionSet::new()
        .run_if_not(conditions::is_paused)
        .run_in_state(SimState::Simulation)
        .with_system(spawning::nearest_spawner_system)
        .into()
}

pub fn family_tree_system_set() -> SystemSet {
    ConditionSet::new()
        .run_in_state(SimState::Simulation)
        .with_system(statistics::family_tree_update)
        .into()
}

pub fn bug_serde_system_set() -> SystemSet {
    ConditionSet::new()
        .run_in_state(SimState::Simulation)
        .with_system(genesis_serde::load_bug_system)
        .with_system(genesis_serde::save_bug_system)
        .into()
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
enum GenesisStage {
    CleanUp,
}

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ColliderMassProperties>()
            .register_type::<ActiveEvents>()
            .add_plugin(attributes::AttributesPlugin)
            .add_plugin(ui::GenesisUiPlugin)
            .add_plugin(behaviour::GenesisBehaviourPlugin)
            .add_plugin(statistics::GenesisStatsPlugin)
            .add_plugin(genesis_ecosystem::EcosystemPlugin)
            .add_enter_system_set(SimState::Simulation, setup::sim_setup_system_set())
            .add_enter_system_set(SimState::Loading, setup::load_simulation_system_set())
            .add_exit_system_set(SimState::Loading, setup::load_simulation_setup_system_set())
            .add_enter_system(SimState::Saving, genesis_serde::save_simulation_system)
            .add_stage_after(
                CoreStage::Update,
                GenesisStage::CleanUp,
                SystemStage::parallel().with_system_set(despawn_system_set()),
            )
            .init_resource::<genesis_serde::LoadedBlueprint>()
            .insert_resource(config::BACKGROUND)
            .init_resource::<SimulationSpeed>()
            .add_system_set(plant_system_set())
            .add_system_set(bug_serde_system_set())
            .add_fixed_timestep(Duration::from_secs(10), "family_tree")
            .add_fixed_timestep(Duration::from_millis(100), "spawner_stats")
            .add_fixed_timestep_system_set("family_tree", 0, family_tree_system_set())
            .add_fixed_timestep_system_set("spawner_stats", 0, nearest_spawner_system_set())
            .add_fixed_timestep_system_set("standard", 0, lifecycle_system_set());
    }
}
