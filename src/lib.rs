use behaviour::lifecycle;
use bevy::{
    prelude::{App, CoreStage, Plugin, StageLabel, SystemSet, SystemStage},
    time::FixedTimestep,
};

mod attributes;
mod behaviour;
mod body;
mod config;
mod ecosystem;
mod mind;
mod setup;
mod spawning;
mod ui;

pub fn slow_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(0.1))
        .with_system(spawning::nearest_spawner_system)
}

pub fn plant_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(spawning::spawn_plant_system)
        .with_system(spawning::update_plant_size)
}

pub fn despawn_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(lifecycle::kill_bug_system)
        .with_system(lifecycle::hatch_egg_system)
        .with_system(spawning::despawn_plants_system)
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

        app.add_plugin(ui::GenesisUiPlugin)
            .add_stage_after(
                CoreStage::Update,
                GenesisStage::CleanUp,
                SystemStage::parallel().with_system_set(despawn_system_set()),
            )
            .insert_resource(config::BACKGROUND)
            .insert_resource(ecosystem::Ecosystem::new(config_instance.world_energy))
            .insert_resource(spawners)
            .insert_resource(spawning::PlantSizeRandomiser::new(
                config_instance.plant_size_range,
            ))
            .add_startup_system_set(setup::setup_system_set())
            .add_system_set(behaviour::behaviour_system_set())
            .add_system_set(plant_system_set())
            .add_system_set(slow_system_set())
            .add_system_set(behaviour::slow_behaviour_system_set())
            .add_system_set(behaviour::egg_spawning_system_set())
            .add_system_set(behaviour::metabolism_system_set());
    }
}
