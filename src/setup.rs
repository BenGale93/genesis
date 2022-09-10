use bevy::prelude::*;
use bevy_rapier2d::prelude::RapierConfiguration;

use crate::ui;

pub fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

pub fn ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(
            // Create a TextBundle that has a Text with a list of sections.
            TextBundle::from_sections([
                TextSection::new(
                    "Energy: ",
                    TextStyle {
                        font: asset_server.load("fonts/calibri.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/calibri.ttf"),
                    font_size: 30.0,
                    color: Color::GOLD,
                }),
            ])
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(ui::EnergyText);
}

pub fn physics_setup(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}
