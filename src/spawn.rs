use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{body, config, ecosystem, food, mind, movement};

fn spawn_bug(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    energy: ecosystem::Energy,
    body: Option<body::BugBody>,
    mind: Option<mind::Mind>,
) {
    let size = 30.0;
    let range = -config::WORLD_SIZE..=config::WORLD_SIZE;
    let mut rng = rand::thread_rng();

    let body_bundle = match body {
        Some(b) => body::BodyBundle::new(b, energy),
        None => body::BodyBundle::random(&mut rng, energy).make_adult(),
    };

    let mind_bundle = match mind {
        Some(m) => mind::MindBundle::new(m),
        None => mind::MindBundle::random(config::INPUT_NEURONS, config::OUTPUT_NEURONS),
    };

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("sprite.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            rng.gen_range(range.clone()),
            rng.gen_range(range),
            0.0,
        )))
        .insert(Collider::capsule(
            Vec2::new(0.0, -6.0),
            Vec2::new(0.0, 6.0),
            9.0,
        ))
        .insert(Velocity::zero())
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(movement::MovementSum::new())
        .insert_bundle(body_bundle)
        .insert_bundle(mind_bundle);
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

fn spawn_food(commands: &mut Commands, asset_server: Res<AssetServer>, energy: ecosystem::Energy) {
    let size = 10.0;
    let range = -config::WORLD_SIZE..=config::WORLD_SIZE;
    let mut rng = rand::thread_rng();

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("food.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            rng.gen_range(range.clone()),
            rng.gen_range(range),
            0.0,
        )))
        .insert(Collider::ball(size / 2.0))
        .insert(Velocity::zero())
        .insert(food::Plant::new(energy));
}

pub fn spawn_food_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ecosystem: ResMut<ecosystem::Ecosystem>,
) {
    if ecosystem.available_energy().as_uint() > 10000 {
        let energy = match ecosystem.request_energy(config::FOOD_ENERGY) {
            None => return,
            Some(e) => e,
        };
        spawn_food(&mut commands, asset_server, energy)
    }
}
