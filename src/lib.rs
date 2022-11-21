use bevy::prelude::{App, CoreStage, Plugin, SystemStage};

mod attributes;
mod behaviour;
mod body;
mod config;
mod ecosystem;
mod mind;
mod setup;
mod ui;

pub struct GenesisPlugin;

impl Plugin for GenesisPlugin {
    fn build(&self, app: &mut App) {
        static CLEAN_UP: &str = "clean_up";
        config::initialize_config();

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
            .add_system_to_stage(CoreStage::Last, ui::save_on_close)
            .add_system_set_to_stage(CLEAN_UP, behaviour::despawn_system_set())
            .insert_resource(ecosystem::Ecosystem::new(
                config::WorldConfig::global().world_energy,
            ));
    }
}
