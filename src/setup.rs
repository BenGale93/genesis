use bevy::prelude::*;
use rand::Rng;

use crate::{body, config, mind};

fn camera_setup(commands: &mut Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn bug_setup(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let mut rng = rand::thread_rng();
    let range = -config::WORLD_SIZE..=config::WORLD_SIZE;
    for _ in 0..config::START_NUM {
        commands
            .spawn()
            .insert(body::BugBody::random(&mut rng))
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

pub fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    camera_setup(&mut commands);
    bug_setup(&mut commands, &asset_server);
}
