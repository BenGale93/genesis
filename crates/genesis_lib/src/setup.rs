use bevy::prelude::{Camera2dBundle, Commands, ResMut, SystemSet, Vec2};
use bevy_rapier2d::prelude::RapierConfiguration;
use iyes_loopless::prelude::*;

use crate::SimState;

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn physics_setup(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}

pub fn sim_setup_system_set() -> SystemSet {
    ConditionSet::new()
        .run_in_state(SimState::Simulation)
        .with_system(camera_setup)
        .with_system(physics_setup)
        .into()
}
