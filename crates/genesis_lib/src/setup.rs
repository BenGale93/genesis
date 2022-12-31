use bevy::prelude::{Camera2dBundle, Commands, ResMut, SystemSet, Vec2};
use bevy_rapier2d::prelude::RapierConfiguration;

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn physics_setup(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}

pub fn setup_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(camera_setup)
        .with_system(physics_setup)
}
