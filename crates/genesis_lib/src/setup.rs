use std::fs;

use bevy::{
    prelude::{
        default, info, AssetServer, Camera2dBundle, Color, Commands, Entity, Query, Res, ResMut,
        SystemSet, Transform, Vec2, With, Without, World,
    },
    scene::DynamicSceneBundle,
};
use bevy_rapier2d::prelude::RapierConfiguration;
use genesis_attributes::Genome;
use genesis_components::{body::OriginalColor, mind, time, Egg, Plant, Size};
use genesis_config as config;
use genesis_ecosystem as ecosystem;
use genesis_spawners::Spawners;
use iyes_loopless::prelude::*;

use crate::{genesis_serde::SimulationSerializer, spawning, statistics, SimState};

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn physics_setup(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}

fn resource_setup(mut commands: Commands) {
    config::initialize_configs(None);

    let config_instance = config::WorldConfig::global();

    let spawners = Spawners::from_configs(&config_instance.spawners).unwrap();
    let plant_spawn_size = spawning::PlantSizeRandomiser::new(config_instance.plant.size_range);
    let ecosystem = ecosystem::Ecosystem::new(config_instance.world_energy);

    commands.insert_resource(spawners);
    commands.insert_resource(plant_spawn_size);
    commands.insert_resource(ecosystem);
    commands.init_resource::<Genome>();
    commands.init_resource::<time::SimulationTime>();
    commands.init_resource::<statistics::FamilyTree>();
    commands.insert_resource(statistics::CountStats::default());
    commands.insert_resource(statistics::BugPerformance::default());
    commands.insert_resource(statistics::EnergyStats::default());
    commands.insert_resource(mind::MindThresholds::new(&config_instance.brain_mutations));
}

pub fn sim_setup_system_set() -> SystemSet {
    ConditionSet::new()
        .run_in_state(SimState::Simulation)
        .run_unless_resource_exists::<time::SimulationTime>()
        .with_system(camera_setup)
        .with_system(physics_setup)
        .with_system(resource_setup)
        .into()
}

fn load_simulation_system(world: &mut World) {
    let path = std::env::current_dir().unwrap();
    let res = rfd::FileDialog::new()
        .set_directory(path)
        .pick_folder()
        .unwrap();
    info!("Loading simulation.");

    let serialize_simulation = fs::read(res.join("resources.ron")).unwrap();
    let simulation: SimulationSerializer = ron::de::from_bytes(&serialize_simulation).unwrap();
    genesis_config::initialize_configs(Some(simulation.config().clone()));
    world.insert_resource(simulation.sim_time().clone());
    world.insert_resource(simulation.ecosystem().clone());
    world.insert_resource(simulation.count_stats().clone());
    world.insert_resource(simulation.energy_stats().clone());
    world.insert_resource(simulation.bug_performance().clone());
    world.insert_resource(simulation.family_tree().clone());

    let config_instance = genesis_config::WorldConfig::global();

    let spawners = Spawners::from_configs(&config_instance.spawners).unwrap();
    let plant_spawn_size = spawning::PlantSizeRandomiser::new(config_instance.plant.size_range);

    world.insert_resource(spawners);
    world.insert_resource(plant_spawn_size);
    world.init_resource::<Genome>();
    world.insert_resource(mind::MindThresholds::new(&config_instance.brain_mutations));
    let asset_server = world.get_resource::<AssetServer>().unwrap();
    world.spawn(DynamicSceneBundle {
        scene: asset_server.load(res.join("scene.scn.ron")),
        ..default()
    });
}

fn mind_layout_system(mut commands: Commands, mind_query: Query<(Entity, &mind::Mind)>) {
    for (entity, mind) in &mind_query {
        let mind_layout = mind::MindLayout::new(mind);
        commands.entity(entity).insert(mind_layout);
    }
}

fn add_missing_components_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    plant_query: Query<(Entity, &Size, &Transform), With<Plant>>,
    egg_query: Query<(Entity, &OriginalColor, &Transform, &Size), With<Egg>>,
    bug_query: Query<(Entity, &Size, &mind::Mind), Without<Egg>>,
) {
    for (entity, size, transform) in &plant_query {
        commands
            .entity(entity)
            .insert(spawning::food_collider(size))
            .insert(spawning::food_sprite_bundle(
                &asset_server,
                size,
                transform.translation,
                Color::GREEN,
            ));
    }

    for (entity, color, transform, size) in &egg_query {
        commands
            .entity(entity)
            .insert(spawning::egg_collider(size))
            .insert(spawning::egg_sprite_bundle(
                &asset_server,
                size,
                color,
                transform.translation,
            ));
    }

    let basic_color = Color::WHITE;
    for (entity, size, mind) in &bug_query {
        commands
            .entity(entity)
            .insert(spawning::bug_collider(size))
            .insert(spawning::bug_sprite_bundle(
                &asset_server,
                size,
                &basic_color,
                mind.color(),
            ));
    }
}

pub fn load_simulation_system_set() -> SystemSet {
    SystemSet::new().with_system(load_simulation_system)
}

pub fn load_simulation_setup_system_set() -> SystemSet {
    ConditionSet::new()
        .with_system(mind_layout_system)
        .with_system(add_missing_components_system)
        .with_system(physics_setup)
        .into()
}
