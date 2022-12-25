use bevy::{
    prelude::{AssetServer, Commands, Entity, Query, Res, ResMut, Transform, Vec3, With, Without},
    sprite::Sprite,
};
use genesis_attributes as attributes;
use genesis_components as components;
use genesis_components::{body, lay::*, mind};
use genesis_config as config;
use genesis_ecosystem as ecosystem;
use genesis_newtype::Probability;
use genesis_spawners::Spawners;

use crate::{setup::MindThresholds, spawning, statistics};

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
    &'a mind::Mind,
    &'a mut body::Vitality,
    &'a attributes::OffspringEnergy,
    &'a components::Generation,
    &'a mut EggsLaid,
    &'a mut components::Relations,
    &'a attributes::Dna,
);

pub fn lay_egg_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    genome: Res<attributes::Genome>,
    mind_thresholds: Res<MindThresholds>,
    mut parent_query: Query<Parent, With<TryingToLay>>,
) {
    let prob = Probability::new(config::WorldConfig::global().mutation_probability).unwrap();
    let mut rng = rand::thread_rng();
    for (
        entity,
        transform,
        mind,
        mut vitality,
        offspring_energy,
        generation,
        mut eggs_laid,
        mut relations,
        dna,
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
        let egg_entity = spawning::spawn_egg(
            &mut commands,
            &asset_server,
            &genome,
            energy,
            location,
            genome.mutate(*dna, &mut rng, &prob),
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

pub fn spawn_egg_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    genome: Res<attributes::Genome>,
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
        let dna = attributes::Dna::new(&genome, &mut rng);
        let mut mind = mind::Mind::random(config::INPUT_NEURONS, config::OUTPUT_NEURONS);
        for _ in 0..config_instance.mutations {
            mind = mind
                .mutate(&mut rng, &Probability::new(1.0).unwrap(), &mind_thresholds)
                .into();
        }
        spawning::spawn_egg(
            &mut commands,
            &asset_server,
            &genome,
            energy,
            location,
            dna,
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
    for (entity, mut egg_energy, mind, sprite, hatch_size, max_size) in hatch_query.iter_mut() {
        commands.entity(entity).remove::<spawning::EggBundle>();
        let hatching_entity = commands.entity(entity);
        let leftover_energy = spawning::spawn_bug(
            &asset_server,
            egg_energy.move_all_energy(),
            (mind.clone(), &sprite.color, hatch_size, max_size),
            hatching_entity,
        );
        ecosystem.return_energy(leftover_energy);
    }
}
