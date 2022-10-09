use bevy::prelude::*;
use bevy_rapier2d::prelude::RapierConfiguration;

pub fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

pub fn physics_setup(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}
