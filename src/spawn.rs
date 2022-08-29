use bevy::prelude::*;
use rand::Rng;

use crate::{body, config, ecosystem, mind};

fn spawn_bug(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    energy: ecosystem::Energy,
    body: Option<body::BugBody>,
    mind: Option<mind::Mind>,
) {
    let mut rng = rand::thread_rng();
    let range = -config::WORLD_SIZE..=config::WORLD_SIZE;

    let body_bundle = match body {
        Some(b) => body::BodyBundle::new(b, energy),
        None => body::BodyBundle::random(&mut rng, energy),
    };

    let mind_bundle = match mind {
        Some(m) => mind::MindBundle::new(m),
        None => mind::MindBundle::random(config::INPUT_NEURONS, config::OUTPUT_NEURONS),
    };

    commands
        .spawn()
        .insert_bundle(body_bundle)
        .insert_bundle(mind_bundle)
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("sprite.png"),
            transform: Transform {
                translation: Vec3::new(rng.gen_range(range.clone()), rng.gen_range(range), 0.0),
                ..default()
            },
            ..default()
        });
}

pub fn spawn_bug_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ecosystem: ResMut<ecosystem::Ecosystem>,
    query: Query<&mind::MindOutput>,
) {
    let bug_num = query.iter().len();

    if bug_num < config::START_NUM {
        let energy = match ecosystem.request_energy(config::START_ENERGY) {
            None => return,
            Some(e) => e,
        };
        spawn_bug(&mut commands, asset_server, energy, None, None)
    }
}
