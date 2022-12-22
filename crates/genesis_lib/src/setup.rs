use bevy::prelude::{Camera2dBundle, Commands, ResMut, Resource, SystemSet, Vec2};
use bevy_rapier2d::prelude::RapierConfiguration;
use derive_more::Deref;
use genesis_brain::BrainMutationThresholds;
use genesis_config as config;

#[derive(Resource, Debug, Deref)]
pub struct MindThresholds(BrainMutationThresholds);

impl MindThresholds {
    pub fn new(brain_config: &config::BrainMutationConfig) -> Self {
        Self(BrainMutationThresholds::new(brain_config))
    }
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn physics_setup(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}

pub fn setup_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(camera_setup)
        .with_system(physics_setup)
}
