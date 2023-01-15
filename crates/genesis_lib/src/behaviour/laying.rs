use bevy::{
    prelude::{AssetServer, Commands, Entity, Query, Res, ResMut, Transform, Vec3, With, Without},
    time::Stopwatch,
};
use genesis_attributes as attributes;
use genesis_components as components;
use genesis_components::{body, lay::*, mind};
use genesis_config as config;
use genesis_ecosystem as ecosystem;
use genesis_newtype::Probability;
use genesis_spawners::Spawners;
use genesis_traits::BehaviourTracker;

use crate::{spawning, statistics};

type LayerTest<'a> = (Entity, &'a mind::MindOutput);

pub fn process_layers_system(
    mut commands: Commands,
    not_laying_query: Query<LayerTest, (Without<TryingToLay>, With<components::Adult>)>,
    laying_query: Query<LayerTest, (With<TryingToLay>, With<components::Adult>)>,
) {
    for (entity, mind_out) in not_laying_query.iter() {
        if mind_out[config::REPRODUCE_INDEX] >= 0.0 {
            commands
                .entity(entity)
                .insert(TryingToLay(Stopwatch::new()));
        }
    }

    for (entity, mind_out) in laying_query.iter() {
        if mind_out[config::REPRODUCE_INDEX] < 0.0 {
            commands.entity(entity).remove::<TryingToLay>();
        }
    }
}

pub fn attempted_to_lay_system(mut bug_query: Query<(&mut TryingToLay, &mut LayingSum)>) {
    let world_config = config::WorldConfig::global();
    for (mut trying_to_lay, mut laying_sum) in bug_query.iter_mut() {
        trying_to_lay.tick(config::BEHAVIOUR_TICK);
        let time_spent = trying_to_lay.elapsed().as_secs_f32();
        if time_spent >= 1.0 {
            laying_sum.add_time(time_spent, world_config.cost_of_lay);
            trying_to_lay.reset();
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
    mind_thresholds: Res<mind::MindThresholds>,
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
    mind_thresholds: Res<mind::MindThresholds>,
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
        let mut mind = mind::Mind::minimal(
            config::INPUT_NEURONS,
            config::OUTPUT_NEURONS,
            &config_instance.starting_synapses,
        );
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
