use bevy::prelude::*;
use rand::Rng;

use crate::{body, config, ecosystem, mind, ui};

pub fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

pub fn bug_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ecosystem: ResMut<ecosystem::Ecosystem>,
) {
    let mut rng = rand::thread_rng();
    let range = -config::WORLD_SIZE..=config::WORLD_SIZE;
    for _ in 0..config::START_NUM {
        let energy = match ecosystem.request_energy(100) {
            None => break,
            Some(e) => e,
        };
        commands
            .spawn()
            .insert_bundle(body::BodyBundle::random(&mut rng, energy))
            .insert_bundle(mind::MindBundle::new(
                config::INPUT_NEURONS,
                config::OUTPUT_NEURONS,
            ))
            .insert_bundle(SpriteBundle {
                texture: asset_server.load("sprite.png"),
                transform: Transform {
                    translation: Vec3::new(
                        rng.gen_range(range.clone()),
                        rng.gen_range(range.clone()),
                        0.0,
                    ),
                    ..default()
                },
                ..default()
            });
    }
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
                ..default()
            }),
        )
        .insert(ui::EnergyText);
}
