use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_rapier2d::prelude::*;
use genesis::{config, resources, setup, systems, ui};

fn main() {
    config::initialize_config();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1000.0))
        .insert_resource(config::BACKGROUND)
        .insert_resource(ui::PanelState::default())
        .insert_resource(WindowDescriptor {
            title: "Genesis".to_string(),
            ..default()
        })
        .insert_resource(resources::Ecosystem::new(
            config::WorldConfig::global().world_energy,
        ))
        .add_startup_system(setup::camera_setup)
        .add_startup_system(setup::physics_setup)
        .add_system_set(systems::interaction_system_set())
        .add_system_set(systems::behavior_system_set())
        .add_system_set(systems::egg_spawning_system_set())
        .add_system_set(systems::plant_spawning_system_set())
        .add_system_set(systems::slow_behavior_system_set())
        .add_system_set(systems::burnt_energy_system_set())
        .add_system_set(systems::selection_system_set())
        .add_system(ui::energy_ui_update_system)
        .run();
}
