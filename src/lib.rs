use std::time::Duration;

use behaviour::lifecycle;
use bevy::prelude::{App, CoreStage, Plugin, StageLabel, SystemSet, SystemStage};
use iyes_loopless::prelude::*;

mod attributes;
mod behaviour;
mod body;
mod config;
mod ecosystem;
mod mind;
mod setup;
mod spawning;
mod ui;

pub fn plant_system_set() -> SystemSet {
    ConditionSet::new()
        .run_if_not(ui::is_paused)
        .with_system(spawning::spawn_plant_system)
        .with_system(spawning::update_plant_size)
        .into()
}

pub fn despawn_system_set() -> SystemSet {
    ConditionSet::new()
        .run_if_not(ui::is_paused)
        .with_system(lifecycle::kill_bug_system)
        .with_system(lifecycle::hatch_egg_system)
        .with_system(spawning::despawn_plants_system)
        .into()
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
enum GenesisStage {
    CleanUp,
}

pub struct GenesisPlugin;

impl Plugin for GenesisPlugin {
    fn build(&self, app: &mut App) {
        config::initialize_config();

        let config_instance = config::WorldConfig::global();

        let spawners = spawning::Spawners::from_configs(&config_instance.spawners).unwrap();
        let plant_spawn_size = spawning::PlantSizeRandomiser::new(config_instance.plant_size_range);
        let ecosystem = ecosystem::Ecosystem::new(config_instance.world_energy);

        app.add_plugin(ui::GenesisUiPlugin)
            .add_plugin(behaviour::GenesisBehaviourPlugin)
            .add_stage_after(
                CoreStage::Update,
                GenesisStage::CleanUp,
                SystemStage::parallel().with_system_set(despawn_system_set()),
            )
            .insert_resource(config::BACKGROUND)
            .insert_resource(spawners)
            .insert_resource(plant_spawn_size)
            .insert_resource(ecosystem)
            .add_startup_system_set(setup::setup_system_set())
            .add_system_set(plant_system_set())
            .add_fixed_timestep(Duration::from_millis(100), "spawner_stats")
            .add_fixed_timestep_system(
                "spawner_stats",
                0,
                spawning::nearest_spawner_system.run_if_not(ui::is_paused),
            );
    }
}
