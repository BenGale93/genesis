use bevy::{
    ecs::system::EntityCommands,
    prelude::{
        default, AssetServer, Bundle, Color, Commands, Entity, Handle, Image, Query, Res, ResMut,
        Transform, Vec2, Vec3, With, Without,
    },
    sprite::{Sprite, SpriteBundle},
};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, Damping, RigidBody, Velocity};
use components::SizeMultiplier;
use genesis_attributes as attributes;
use genesis_components as components;
use genesis_components::{body, eat, grow, lay::*, mind, see, time, BurntEnergy};
use genesis_config as config;
use genesis_ecosystem as ecosystem;
use genesis_newtype::Probability;
use genesis_spawners::Spawners;

use crate::{setup::MindThresholds, statistics};

type LayerTest<'a> = (Entity, &'a mind::MindOutput);

pub fn process_layers_system(
    mut commands: Commands,
    not_laying_query: Query<LayerTest, (Without<TryingToLay>, With<components::Adult>)>,
    laying_query: Query<LayerTest, (With<TryingToLay>, With<components::Adult>)>,
) {
    for (entity, mind_out) in not_laying_query.iter() {
        if mind_out[config::REPRODUCE_INDEX] >= 0.0 {
            commands.entity(entity).insert(TryingToLay);
        }
    }

    for (entity, mind_out) in laying_query.iter() {
        if mind_out[config::REPRODUCE_INDEX] < 0.0 {
            commands.entity(entity).remove::<TryingToLay>();
        }
    }
}

type Parent<'a> = (
    Entity,
    &'a Transform,
    &'a attributes::Genome,
    &'a mind::Mind,
    &'a mut body::Vitality,
    &'a attributes::OffspringEnergy,
    &'a components::Generation,
    &'a mut EggsLaid,
    &'a mut components::Relations,
);

pub fn lay_egg_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mind_thresholds: Res<MindThresholds>,
    mut parent_query: Query<Parent, With<TryingToLay>>,
) {
    let prob = Probability::new(config::WorldConfig::global().mutation_probability).unwrap();
    let mut rng = rand::thread_rng();
    for (
        entity,
        transform,
        genome,
        mind,
        mut vitality,
        offspring_energy,
        generation,
        mut eggs_laid,
        mut relations,
    ) in parent_query.iter_mut()
    {
        let egg_energy =
            (vitality.energy_store().energy_limit() as f32 * **offspring_energy) as usize;
        if vitality.energy_store().amount() < egg_energy {
            continue;
        }
        let energy = vitality.take_energy(egg_energy);
        let location = egg_position(transform);
        eggs_laid.0 += 1;
        let egg_entity = spawn_egg(
            &mut commands,
            &asset_server,
            energy,
            location,
            genome.mutate(&mut rng, &prob),
            mind.mutate(&mut rng, &prob, &mind_thresholds).into(),
            *generation + 1.into(),
            Some(entity),
        );
        relations.add_child(egg_entity);
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

type BugParts<'a> = (
    attributes::Genome,
    mind::Mind,
    &'a Color,
    &'a attributes::HatchSize,
    &'a attributes::MaxSize,
);

fn spawn_bug(
    asset_server: &Res<AssetServer>,
    energy: ecosystem::Energy,
    bug_parts: BugParts,
    mut hatching_entity: EntityCommands,
) -> ecosystem::Energy {
    let (bug_body, mind, color, hatch_size, max_size) = bug_parts;
    let mind_bundle = mind::MindBundle::new(&mind);

    let original_color = body::OriginalColor(mind.color());
    // Allows selected eggs to remain selected on hatching
    let current_color = if *color == Color::RED {
        *color
    } else {
        original_color.0
    };

    let size = body::Size::new(**hatch_size, **max_size);
    let (vitality, leftover_energy) = body::Vitality::new(size, energy);

    let sprite_image: Handle<Image> = asset_server.load("sprite.png");
    let sprite = Sprite {
        custom_size: Some(vitality.size().sprite()),
        color: current_color,
        ..default()
    };

    hatching_entity
        .insert(sprite_image)
        .insert(sprite)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(vitality.size().collider())
        .insert(SizeMultiplier::new(vitality.size().current_size()))
        .insert(components::Juvenile)
        .insert(original_color)
        .insert(bug_body)
        .insert(vitality)
        .insert(mind_bundle)
        .insert(see::Vision::new())
        .insert(time::Age::default())
        .insert(time::Heart::new())
        .insert(time::InternalTimer::new())
        .insert(components::MovementSum::new())
        .insert(components::ThinkingSum::new())
        .insert(eat::EatingSum::new())
        .insert(grow::GrowingSum::new())
        .insert(grow::SizeSum::new())
        .insert(eat::EnergyConsumed(0))
        .insert(EggsLaid(0));

    leftover_energy
}

#[derive(Bundle)]
struct EggBundle {
    pub egg: components::Egg,
    pub egg_energy: ecosystem::EggEnergy,
    pub sprite: Sprite,
    pub handle: Handle<Image>,
    pub original_color: body::OriginalColor,
    pub collider: Collider,
    pub age: time::Age,
}

fn spawn_egg(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    energy: ecosystem::Energy,
    location: Vec3,
    genome: attributes::Genome,
    mind: mind::Mind,
    generation: components::Generation,
    parent_id: Option<Entity>,
) -> Entity {
    let size = 16.0;

    let attribute_bundle = attributes::AttributeBundle::new(&genome);
    let original_color = body::OriginalColor(Color::WHITE);
    let sprite = SpriteBundle {
        texture: asset_server.load("egg.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(size, size)),
            color: original_color.0,
            ..default()
        },
        transform: Transform::from_translation(location),
        ..default()
    };

    let mut egg_entity = commands.spawn(sprite);
    let entity = egg_entity.id();

    egg_entity
        .insert(RigidBody::Dynamic)
        .insert(Damping {
            linear_damping: 1.0,
            angular_damping: 1.0,
        })
        .insert(Velocity::zero())
        .insert(Collider::ball(size / 2.0))
        .insert(components::Egg)
        .insert(attribute_bundle)
        .insert(ecosystem::EggEnergy(energy))
        .insert(original_color)
        .insert(genome)
        .insert(components::Relations::new(
            (entity, mind.color()),
            parent_id,
        ))
        .insert(mind)
        .insert(time::Age::default())
        .insert(generation)
        .insert(BurntEnergy::new());

    entity
}

pub fn spawn_egg_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ecosystem: ResMut<ecosystem::Ecosystem>,
    spawners: Res<Spawners>,
    count_stats: Res<statistics::CountStats>,
    performance_stats: Res<statistics::BugPerformance>,
    mind_thresholds: Res<MindThresholds>,
) {
    let config_instance = config::WorldConfig::global();
    let bug_num = count_stats.current_organisms();
    let max_generation = performance_stats.current_max_generation();

    if (spawners.space_for_organisms(config_instance.minimum_number))
        || (bug_num < config_instance.start_num && max_generation < config::GENERATION_SWITCH)
    {
        let Some(energy) = ecosystem.request_energy(config_instance.start_energy) else { return };
        let mut rng = rand::thread_rng();
        let location = spawners.random_organism_position(&mut rng);
        let genome = attributes::Genome::new(&mut rng);
        let mut mind = mind::Mind::random(config::INPUT_NEURONS, config::OUTPUT_NEURONS);
        for _ in 0..config_instance.mutations {
            mind = mind
                .mutate(&mut rng, &Probability::new(1.0).unwrap(), &mind_thresholds)
                .into();
        }
        spawn_egg(
            &mut commands,
            &asset_server,
            energy,
            location,
            genome,
            mind,
            components::Generation(0),
            None,
        );
    }
}

type EggQuery<'a> = (
    Entity,
    &'a mut ecosystem::EggEnergy,
    &'a mind::Mind,
    &'a attributes::Genome,
    &'a Sprite,
    &'a attributes::HatchSize,
    &'a attributes::MaxSize,
);

pub fn hatch_egg_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ecosystem: ResMut<ecosystem::Ecosystem>,
    mut hatch_query: Query<EggQuery, With<components::Hatching>>,
) {
    for (entity, mut egg_energy, mind, genome, sprite, hatch_size, max_size) in
        hatch_query.iter_mut()
    {
        commands.entity(entity).remove::<EggBundle>();
        let hatching_entity = commands.entity(entity);
        let leftover_energy = spawn_bug(
            &asset_server,
            egg_energy.move_all_energy(),
            (
                genome.clone(),
                mind.clone(),
                &sprite.color,
                hatch_size,
                max_size,
            ),
            hatching_entity,
        );
        ecosystem.return_energy(leftover_energy);
    }
}
