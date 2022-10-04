use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{attributes, body, config, ecosystem, lifecycle, mind, movement, sight};

type BugParts<'a> = (
    body::BugBody,
    mind::Mind,
    &'a Transform,
    lifecycle::Generation,
);

pub fn spawn_bug(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    energy: ecosystem::Energy,
    bug_parts: BugParts,
) {
    let size = 30.0;

    let (bug_body, mind, transform, generation) = bug_parts;
    let mind_bundle = mind::MindBundle::new(mind);
    let transform_bundle = TransformBundle::from(*transform);

    let attribute_bundle = attributes::AttributeBundle::new(bug_body.genome());

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
        .insert(Damping {
            linear_damping: 1.0,
            angular_damping: 1.0,
        })
        .insert_bundle(transform_bundle)
        .insert(Collider::capsule(
            Vec2::new(0.0, -6.0),
            Vec2::new(0.0, 6.0),
            9.0,
        ))
        .insert(Velocity::zero())
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(movement::MovementSum::new())
        .insert(bug_body)
        .insert(body::Age::default())
        .insert(lifecycle::Juvenile)
        .insert(sight::Vision::new())
        .insert(body::Vitality::new(energy))
        .insert(body::BurntEnergy::new())
        .insert(body::Heart::new())
        .insert(body::InternalTimer::new())
        .insert(generation)
        .insert_bundle(attribute_bundle)
        .insert_bundle(mind_bundle);
}

pub fn spawn_egg_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ecosystem: ResMut<ecosystem::Ecosystem>,
    query: Query<&mind::MindOutput>,
) {
    let range = -config::WORLD_SIZE..=config::WORLD_SIZE;
    let mut rng = rand::thread_rng();
    let bug_num = query.iter().len();

    if bug_num < config::START_NUM {
        let energy = match ecosystem.request_energy(config::START_ENERGY) {
            None => return,
            Some(e) => e,
        };
        let location = Vec3::new(rng.gen_range(range.clone()), rng.gen_range(range), 0.0);
        let bug_body = body::BugBody::random(&mut rng);
        let mind = mind::Mind::random(config::INPUT_NEURONS, config::OUTPUT_NEURONS);
        spawn_egg(
            &mut commands,
            &asset_server,
            energy,
            location,
            bug_body,
            mind,
            lifecycle::Generation(0),
        );
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
        .insert(Damping {
            linear_damping: 1.0,
            angular_damping: 1.0,
        })
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
    mind: mind::Mind,
    generation: lifecycle::Generation,
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
        .insert(Damping {
            linear_damping: 1.0,
            angular_damping: 1.0,
        })
        .insert_bundle(TransformBundle::from(Transform::from_translation(location)))
        .insert_bundle(attribute_bundle)
        .insert(Collider::ball(size / 2.0))
        .insert(Velocity::zero())
        .insert(bug_body)
        .insert(mind)
        .insert(generation)
        .insert(body::Age::default())
        .insert(body::BurntEnergy::new())
        .insert(body::Vitality::new(energy));
}
