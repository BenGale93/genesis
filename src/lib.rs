use bevy::prelude::{App, Plugin};

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
        config::initialize_config();

        app.insert_resource(config::BACKGROUND)
            .insert_resource(ui::PanelState::default())
            .add_startup_system_set(setup::setup_system_set())
            .add_system_set(ui::interaction_system_set())
            .add_system_set(ui::selection_system_set())
            .add_system_set(behaviour::behaviour_system_set())
            .add_system_set(behaviour::egg_spawning_system_set())
            .add_system_set(behaviour::plant_spawning_system_set())
            .add_system_set(behaviour::slow_behaviour_system_set())
            .add_system_set(behaviour::metabolism_system_set())
            .insert_resource(ecosystem::Ecosystem::new(
                config::WorldConfig::global().world_energy,
            ));
    }
}
