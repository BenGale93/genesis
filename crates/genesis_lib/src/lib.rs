#![warn(clippy::all, clippy::nursery)]
#![feature(is_some_and)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::similar_names)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
use std::time::Duration;

use behaviour::lifecycle;
use bevy::prelude::{App, CoreStage, Plugin, StageLabel, SystemSet, SystemStage};
use genesis_spawners::Spawners;
use iyes_loopless::prelude::*;

mod ancestors;
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
        config::initialize_configs();

        let config_instance = config::WorldConfig::global();

        let spawners = Spawners::from_configs(&config_instance.spawners).unwrap();
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
            .insert_resource(ancestors::FamilyTree::default())
            .insert_resource(spawners)
            .insert_resource(plant_spawn_size)
            .insert_resource(ecosystem)
            .add_startup_system_set(setup::setup_system_set())
            .add_system_set(plant_system_set())
            .add_fixed_timestep(Duration::from_secs(10), "family_tree")
            .add_fixed_timestep_system("family_tree", 0, ancestors::family_tree_update)
            .add_fixed_timestep(Duration::from_millis(100), "spawner_stats")
            .add_fixed_timestep_system(
                "spawner_stats",
                0,
                spawning::nearest_spawner_system.run_if_not(ui::is_paused),
            );
    }
}
