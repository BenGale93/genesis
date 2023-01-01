use bevy::prelude::{Camera2dBundle, Commands, ResMut, SystemSet, Vec2};
use bevy_rapier2d::prelude::RapierConfiguration;
use genesis_attributes::Genome;
use genesis_components::{mind, time};
use genesis_config as config;
use genesis_ecosystem as ecosystem;
use genesis_spawners::Spawners;
use iyes_loopless::prelude::*;

use crate::{spawning, SimState};

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn physics_setup(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}

fn resource_setup(mut commands: Commands) {
    config::initialize_configs();

    let config_instance = config::WorldConfig::global();

    let spawners = Spawners::from_configs(&config_instance.spawners).unwrap();
    let plant_spawn_size = spawning::PlantSizeRandomiser::new(config_instance.plant_size_range);
    let ecosystem = ecosystem::Ecosystem::new(config_instance.world_energy);

    commands.insert_resource(spawners);
    commands.insert_resource(plant_spawn_size);
    commands.insert_resource(ecosystem);
    commands.init_resource::<Genome>();
    commands.init_resource::<time::SimulationTime>();
    commands.insert_resource(mind::MindThresholds::new(&config_instance.brain_mutations));
}

pub fn sim_setup_system_set() -> SystemSet {
    ConditionSet::new()
        .run_in_state(SimState::Simulation)
        .with_system(camera_setup)
        .with_system(physics_setup)
        .with_system(resource_setup)
        .into()
}
