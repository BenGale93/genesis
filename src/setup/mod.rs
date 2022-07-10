use bevy::prelude::*;

use crate::components;

pub fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn()
        .insert(components::Bug)
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("sprite.png"),
            ..default()
        });
}
