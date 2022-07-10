use bevy::prelude::*;
use genesis::{config, setup, systems};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup::startup)
        .insert_resource(config::BACKGROUND)
        .insert_resource(WindowDescriptor {
            title: "Genesis".to_string(),
            ..default()
        })
        .add_system_set(systems::moving_bug_system_set())
        .add_system_set(systems::moving_camera_system_set())
        .run();
}
