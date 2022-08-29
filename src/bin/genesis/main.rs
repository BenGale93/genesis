use bevy::prelude::*;
use genesis::{config, resources, setup, systems, ui};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(config::BACKGROUND)
        .insert_resource(WindowDescriptor {
            title: "Genesis".to_string(),
            ..default()
        })
        .insert_resource(resources::Ecosystem::new(2000))
        .add_startup_system(setup::camera_setup)
        .add_startup_system(setup::ui_setup)
        .add_system_set(systems::moving_camera_system_set())
        .add_system_set(systems::behavior_system_set())
        .add_system_set(systems::bug_spawning_system_set())
        .add_system(ui::energy_ui_update_system)
        .run();
}
