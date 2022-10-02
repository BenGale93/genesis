use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{attributes, body, config, ecosystem, lifecycle, mind, movement, sight};

fn spawn_bug(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    energy: ecosystem::Energy,
    body_opt: Option<body::BugBody>,
    mind: Option<mind::Mind>,
) {
    let size = 30.0;
    let range = -config::WORLD_SIZE..=config::WORLD_SIZE;
    let mut rng = rand::thread_rng();

    let adult = body_opt.is_none();

    let bug_body = match body_opt {
        Some(b) => b,
        None => body::BugBody::random(&mut rng),
    };

    let attribute_bundle = attributes::AttributeBundle::new(bug_body.genome());

    let mind_bundle = match mind {
        Some(m) => mind::MindBundle::new(m),
        None => mind::MindBundle::random(config::INPUT_NEURONS, config::OUTPUT_NEURONS),
    };

    let mut entity = commands.spawn();

    if adult {
        entity
            .insert(body::Age::new(attribute_bundle.adult_age.value()))
            .insert(lifecycle::Adult);
    } else {
        entity
            .insert(body::Age::default())
            .insert(lifecycle::Juvenile);
    }

    entity
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
        .insert(bug_body)
        .insert(sight::Vision::new())
        .insert(body::Vitality::new(energy))
        .insert(body::BurntEnergy::new())
        .insert(body::Heart::new())
        .insert(body::InternalTimer::new())
        .insert_bundle(attribute_bundle)
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

fn spawn_plant(commands: &mut Commands, asset_server: Res<AssetServer>, energy: ecosystem::Energy) {
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
        .insert(ecosystem::Plant::new(energy));
}

pub fn spawn_plant_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ecosystem: ResMut<ecosystem::Ecosystem>,
) {
    if ecosystem.available_energy().as_uint() > 10000 {
        let energy = match ecosystem.request_energy(config::PLANT_ENERGY) {
            None => return,
            Some(e) => e,
        };
        spawn_plant(&mut commands, asset_server, energy)
    }
}

pub fn kill_bug_system(
    mut commands: Commands,
    query: Query<(Entity, &body::Vitality, &attributes::DeathAge, &body::Age)>,
) {
    for (entity, vitality, death_age, age) in query.iter() {
        if vitality.health().amount() == 0 || death_age.value() < age.elapsed_secs() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn spawn_egg(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    energy: ecosystem::Energy,
    location: Vec3,
    bug_body: body::BugBody,
) {
    let size = 16.0;

    let attribute_bundle = attributes::EggAttributeBundle::new(bug_body.genome());

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("egg.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert_bundle(TransformBundle::from(Transform::from_translation(location)))
        .insert_bundle(attribute_bundle)
        .insert(Collider::ball(size / 2.0))
        .insert(Velocity::zero())
        .insert(bug_body)
        .insert(body::Age::default())
        .insert(body::BurntEnergy::new())
        .insert(body::Vitality::new(energy));
}
