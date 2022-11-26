use bevy::{
    prelude::{App, CoreStage, Plugin, SystemSet, SystemStage},
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

pub fn time_step_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(config::TIME_STEP as f64))
        .with_system(spawning::spawn_plant_system)
}

pub struct GenesisPlugin;

impl Plugin for GenesisPlugin {
    fn build(&self, app: &mut App) {
        static CLEAN_UP: &str = "clean_up";
        config::initialize_config();

        let config_instance = config::WorldConfig::global();

        let spawners = spawning::Spawners::from_configs(&config_instance.spawners).unwrap();

        app.add_stage_after(CoreStage::Update, CLEAN_UP, SystemStage::parallel())
            .insert_resource(config::BACKGROUND)
            .insert_resource(ui::AverageAttributeStatistics::default())
            .insert_resource(ui::CountStatistics::default())
            .insert_resource(ui::BugPerformanceStatistics::default())
            .insert_resource(ui::EnergyStatistics::default())
            .insert_resource(ui::EntityPanelState::default())
            .insert_resource(ui::GlobalPanelState::default())
            .add_startup_system_set(setup::setup_system_set())
            .add_system_set(ui::interaction_system_set())
            .add_system_set(ui::selection_system_set())
            .add_system_set(behaviour::time_step_system_set())
            .add_system_set(behaviour::egg_spawning_system_set())
            .add_system_set(behaviour::slow_behaviour_system_set())
            .add_system_set(behaviour::metabolism_system_set())
            .add_system_set(ui::global_statistics_system_set())
            .add_system_set(ui::regular_saver_system_set())
            .add_system_set(slow_system_set())
            .add_system_set(time_step_system_set())
            .add_system_to_stage(CoreStage::Last, ui::save_on_close)
            .add_system_set_to_stage(CLEAN_UP, behaviour::despawn_system_set())
            .insert_resource(ecosystem::Ecosystem::new(config_instance.world_energy))
            .insert_resource(spawners)
            .insert_resource(spawning::PlantSizeRandomiser::new(
                config_instance.plant_size_range,
            ));
    }
}
