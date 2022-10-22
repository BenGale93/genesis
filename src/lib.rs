use bevy::prelude::{App, Plugin};

mod attributes;
mod body;
mod config;
mod ecosystem;
mod interaction;
mod lifecycle;
mod mind;
mod movement;
mod setup;
mod sight;
mod spawn;
mod systems;
mod ui;

pub struct GenesisPlugin;

impl Plugin for GenesisPlugin {
    fn build(&self, app: &mut App) {
        config::initialize_config();

        app.insert_resource(config::BACKGROUND)
            .insert_resource(ui::PanelState::default())
            .add_startup_system(setup::camera_setup)
            .add_startup_system(setup::physics_setup)
            .add_system_set(systems::interaction_system_set())
            .add_system_set(systems::behavior_system_set())
            .add_system_set(systems::egg_spawning_system_set())
            .add_system_set(systems::plant_spawning_system_set())
            .add_system_set(systems::slow_behavior_system_set())
            .add_system_set(systems::burnt_energy_system_set())
            .add_system_set(systems::selection_system_set())
            .insert_resource(ecosystem::Ecosystem::new(
                config::WorldConfig::global().world_energy,
            ))
            .add_system(ui::energy_ui_update_system);
    }
}
