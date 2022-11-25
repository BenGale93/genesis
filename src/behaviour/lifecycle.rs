use bevy::{
    prelude::{
        default, AssetServer, Color, Commands, Component, DespawnRecursiveExt, Entity, Query, Res,
        ResMut, Transform, Vec2, Vec3, With, Without,
    },
    sprite::{Sprite, SpriteBundle},
    transform::TransformBundle,
};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, Damping, RigidBody, Velocity};
use derive_more::{Add, Deref, DerefMut, From};
use genesis_util::Probability;

use super::{eating, growth, metabolism, movement, sight, spawning, thinking};
use crate::{attributes, behaviour::timers, body, config, ecosystem, mind, ui};

#[derive(Component, Debug)]
pub struct Hatching;

#[derive(Component, Debug)]
pub struct Juvenile;

#[derive(Component, Debug)]
pub struct Adult;

#[derive(
    Component, Debug, Deref, DerefMut, Clone, Copy, From, Add, Ord, PartialOrd, Eq, PartialEq,
)]
pub struct Generation(pub usize);

pub fn transition_to_adult_system(
    mut commands: Commands,
    bug_query: Query<(Entity, &timers::Age, &attributes::AdultAge), With<Juvenile>>,
) {
    for (entity, age, adult_age) in bug_query.iter() {
        if age.elapsed_secs() > **adult_age {
            commands.entity(entity).remove::<Juvenile>().insert(Adult);
        }
    }
}

pub fn transition_to_hatching_system(
    mut commands: Commands,
    egg_query: Query<(Entity, &timers::Age, &attributes::HatchAge), Without<Hatching>>,
) {
    for (entity, age, hatch_age) in egg_query.iter() {
        if age.elapsed_secs() > **hatch_age {
            commands.entity(entity).insert(Hatching);
        }
    }
}

type Egg<'a> = (
    Entity,
    &'a mut EggEnergy,
    &'a Transform,
    &'a mind::Mind,
    &'a body::BugBody,
    &'a Generation,
);

pub fn hatch_egg_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut hatch_query: Query<Egg, With<Hatching>>,
) {
    for (entity, mut egg_energy, transform, mind, body, generation) in hatch_query.iter_mut() {
        commands.entity(entity).despawn_recursive();
        spawn_bug(
            &mut commands,
            &asset_server,
            egg_energy.move_all_energy(),
            (body.clone(), mind.clone(), transform, *generation),
        )
    }
}

#[derive(Component)]
pub struct TryingToLay;

type LayerTest<'a> = (Entity, &'a mind::MindOutput, &'a attributes::LayEggBoundary);

pub fn process_layers_system(
    mut commands: Commands,
    not_laying_query: Query<LayerTest, (Without<TryingToLay>, With<Adult>)>,
    laying_query: Query<LayerTest, (With<TryingToLay>, With<Adult>)>,
) {
    for (entity, mind_out, boundary) in not_laying_query.iter() {
        if mind_out[config::REPRODUCE_INDEX] > **boundary {
            commands.entity(entity).insert(TryingToLay);
        }
    }

    for (entity, mind_out, boundary) in laying_query.iter() {
        if mind_out[config::REPRODUCE_INDEX] <= **boundary {
            commands.entity(entity).remove::<TryingToLay>();
        }
    }
}

fn egg_position(parent_transform: &Transform) -> Vec3 {
    let separation = 20.0;
    let mut egg_pos = parent_transform.translation;
    let angle = parent_transform.rotation.z.asin() * 2.0;
    let (s, c) = angle.sin_cos();

    egg_pos.y -= separation * c;
    egg_pos.x += separation * s;

    egg_pos
}

#[derive(Component, Copy, Clone, Debug, Deref, Ord, PartialEq, Eq, PartialOrd)]
pub struct EggsLaid(pub usize);

type Parent<'a> = (
    &'a Transform,
    &'a body::BugBody,
    &'a mind::Mind,
    &'a attributes::MutationProbability,
    &'a mut body::Vitality,
    &'a attributes::OffspringEnergy,
    &'a Generation,
    &'a mut EggsLaid,
);

pub fn lay_egg_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut parent_query: Query<Parent, With<TryingToLay>>,
) {
    let mut rng = rand::thread_rng();
    for (
        transform,
        bug_body,
        mind,
        prob,
        mut vitality,
        offspring_energy,
        generation,
        mut eggs_laid,
    ) in parent_query.iter_mut()
    {
        if vitality.energy_store().amount() < **offspring_energy {
            continue;
        }
        let energy = vitality.take_energy(**offspring_energy);
        let location = egg_position(transform);
        let offspring_body = bug_body.mutate(&mut rng, **prob);
        let offspring_mind = mind.mutate(&mut rng, **prob).into();
        eggs_laid.0 += 1;
        spawn_egg(
            &mut commands,
            &asset_server,
            energy,
            location,
            offspring_body,
            offspring_mind,
            *generation + 1.into(),
        );
    }
}

type BugParts<'a> = (body::BugBody, mind::Mind, &'a Transform, Generation);

fn spawn_bug(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    energy: ecosystem::Energy,
    bug_parts: BugParts,
) {
    let (bug_body, mind, transform, generation) = bug_parts;
    let mind_bundle = mind::MindBundle::new(mind);
    let transform_bundle = TransformBundle::from(*transform);

    let attribute_bundle = attributes::AttributeBundle::new(bug_body.genome());

    let original_color = body::OriginalColor(Color::WHITE);

    let size = body::Size::new(*attribute_bundle.hatch_size, *attribute_bundle.max_size);

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("sprite.png"),
            sprite: Sprite {
                custom_size: Some(size.sprite()),
                color: original_color.0,
                ..default()
            },
            ..default()
        })
        .insert(original_color)
        .insert(RigidBody::Dynamic)
        .insert(Damping {
            linear_damping: 1.0,
            angular_damping: 1.0,
        })
        .insert(transform_bundle)
        .insert(size.collider())
        .insert(Velocity::zero())
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(movement::MovementSum::new())
        .insert(bug_body)
        .insert(timers::Age::default())
        .insert(Juvenile)
        .insert(sight::Vision::new())
        .insert(body::Vitality::new(size, energy))
        .insert(metabolism::BurntEnergy::new())
        .insert(timers::Heart::new())
        .insert(timers::InternalTimer::new())
        .insert(thinking::ThinkingSum::new())
        .insert(eating::EatingSum::new())
        .insert(growth::GrowingSum::new())
        .insert(growth::SizeSum::new())
        .insert(generation)
        .insert(eating::EnergyConsumed(0))
        .insert(EggsLaid(0))
        .insert(attribute_bundle)
        .insert(mind_bundle);
}

pub fn kill_bug_system(
    mut commands: Commands,
    mut ecosystem: ResMut<ecosystem::Ecosystem>,
    mut query: Query<(
        Entity,
        &mut body::Vitality,
        &attributes::DeathAge,
        &timers::Age,
    )>,
) {
    for (entity, mut vitality, death_age, age) in query.iter_mut() {
        if vitality.health().amount() == 0 || **death_age < age.elapsed_secs() {
            ecosystem.return_energy(vitality.take_all_energy());
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Component)]
pub struct EggEnergy(ecosystem::Energy);

impl EggEnergy {
    #[must_use]
    pub fn move_all_energy(&mut self) -> ecosystem::Energy {
        self.0.take_energy(self.0.amount())
    }
}

fn spawn_egg(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    energy: ecosystem::Energy,
    location: Vec3,
    bug_body: body::BugBody,
    mind: mind::Mind,
    generation: Generation,
) {
    let size = 16.0;

    let attribute_bundle = attributes::EggAttributeBundle::new(bug_body.genome());
    let original_color = body::OriginalColor(Color::WHITE);

    commands
        .spawn(EggEnergy(energy))
        .insert(SpriteBundle {
            texture: asset_server.load("egg.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(size, size)),
                color: original_color.0,
                ..default()
            },
            ..default()
        })
        .insert(original_color)
        .insert(RigidBody::Dynamic)
        .insert(Damping {
            linear_damping: 1.0,
            angular_damping: 1.0,
        })
        .insert(TransformBundle::from(Transform::from_translation(location)))
        .insert(attribute_bundle)
        .insert(Collider::ball(size / 2.0))
        .insert(Velocity::zero())
        .insert(bug_body)
        .insert(mind)
        .insert(generation)
        .insert(timers::Age::default())
        .insert(metabolism::BurntEnergy::new());
}

pub fn spawn_egg_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ecosystem: ResMut<ecosystem::Ecosystem>,
    spawners: Res<spawning::Spawners>,
    count_stats: Res<ui::CountStatistics>,
    performance_stats: Res<ui::BugPerformanceStatistics>,
) {
    let config_instance = config::WorldConfig::global();
    let bug_num = count_stats.current_organisms();
    let max_generation = performance_stats.current_max_generation();

    if (bug_num < config_instance.minimum_number)
        || (bug_num < config_instance.start_num && max_generation < config::GENERATION_SWITCH)
    {
        let energy = match ecosystem.request_energy(config_instance.start_energy) {
            None => return,
            Some(e) => e,
        };
        let mut rng = rand::thread_rng();
        let location = spawners.random_organism_position(&mut rng);
        let bug_body = body::BugBody::random(&mut rng);
        let mut mind = mind::Mind::random(config::INPUT_NEURONS, config::OUTPUT_NEURONS);
        for _ in 0..config_instance.mutations {
            mind = mind.mutate(&mut rng, Probability::new(1.0).unwrap()).into();
        }
        spawn_egg(
            &mut commands,
            &asset_server,
            energy,
            location,
            bug_body,
            mind,
            Generation(0),
        );
    }
}

fn spawn_plant(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    energy: ecosystem::Energy,
    location: Vec3,
) {
    let size = 10.0;
    let original_color = body::OriginalColor(Color::GREEN);

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("food.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(size, size)),
                color: original_color.0,
                ..default()
            },
            ..default()
        })
        .insert(original_color)
        .insert(RigidBody::Dynamic)
        .insert(Damping {
            linear_damping: 1.0,
            angular_damping: 1.0,
        })
        .insert(TransformBundle::from(Transform::from_translation(location)))
        .insert(Collider::ball(size / 2.0))
        .insert(Velocity::zero())
        .insert(ecosystem::Plant::new(energy));
}

pub fn spawn_plant_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ecosystem: ResMut<ecosystem::Ecosystem>,
    spawners: Res<spawning::Spawners>,
) {
    let config_instance = config::WorldConfig::global();
    let available_energy = ecosystem.available_energy().amount();

    if available_energy > (config_instance.start_num * config_instance.start_energy) {
        let energy = match ecosystem.request_energy(config_instance.plant_energy) {
            None => return,
            Some(e) => e,
        };
        let mut rng = rand::thread_rng();
        let location = spawners.random_food_position(&mut rng);
        spawn_plant(&mut commands, asset_server, energy, location)
    }
}
