use std::time::Duration;

use bevy::prelude::{App, CoreStage, Plugin, StageLabel, SystemSet, SystemStage};
use genesis_attributes as attributes;
use genesis_config as config;
use genesis_ecosystem as ecosystem;
use genesis_spawners::Spawners;
use iyes_loopless::prelude::*;

use crate::{behaviour, bug_serde, lifecycle, setup, spawning, statistics, ui, SimState};

pub fn plant_system_set() -> SystemSet {
    ConditionSet::new()
        .run_if_not(ui::is_paused)
        .run_in_state(SimState::Simulation)
        .with_system(spawning::spawn_plant_system)
        .with_system(spawning::update_plant_size)
        .into()
}

pub fn despawn_system_set() -> SystemSet {
    ConditionSet::new()
        .run_if_not(ui::is_paused)
        .run_in_state(SimState::Simulation)
        .with_system(lifecycle::kill_bug_system)
        .with_system(behaviour::laying::hatch_egg_system)
        .with_system(spawning::despawn_plants_system)
        .into()
}

pub fn lifecycle_system_set() -> SystemSet {
    ConditionSet::new()
        .label("lifecycle")
        .run_if_not(ui::is_paused)
        .run_in_state(SimState::Simulation)
        .with_system(lifecycle::transition_to_adult_system)
        .with_system(lifecycle::transition_to_hatching_system)
        .into()
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
enum GenesisStage {
    CleanUp,
}

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        config::initialize_configs();

        let config_instance = config::WorldConfig::global();

        let spawners = Spawners::from_configs(&config_instance.spawners).unwrap();
        let plant_spawn_size = spawning::PlantSizeRandomiser::new(config_instance.plant_size_range);
        let ecosystem = ecosystem::Ecosystem::new(config_instance.world_energy);

        app.add_plugin(attributes::AttributesPlugin)
            .add_plugin(ui::GenesisUiPlugin)
            .add_plugin(behaviour::GenesisBehaviourPlugin)
            .add_stage_after(
                CoreStage::Update,
                GenesisStage::CleanUp,
                SystemStage::parallel().with_system_set(despawn_system_set()),
            )
            .init_resource::<bug_serde::LoadedBlueprint>()
            .init_resource::<statistics::FamilyTree>()
            .insert_resource(config::BACKGROUND)
            .insert_resource(spawners)
            .insert_resource(plant_spawn_size)
            .insert_resource(ecosystem)
            .add_startup_system_set(setup::sim_setup_system_set())
            .add_system_set(plant_system_set())
            .add_fixed_timestep(Duration::from_secs(10), "family_tree")
            .add_fixed_timestep_system("family_tree", 0, statistics::family_tree_update)
            .add_fixed_timestep(Duration::from_millis(100), "spawner_stats")
            .add_fixed_timestep_system(
                "spawner_stats",
                0,
                spawning::nearest_spawner_system.run_if_not(ui::is_paused),
            )
            .add_fixed_timestep_system_set("standard", 0, lifecycle_system_set());
    }
}
