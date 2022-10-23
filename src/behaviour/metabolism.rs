use bevy::{
    prelude::{Component, Query, Res, ResMut},
    time::Time,
};
use derive_more::{Add, Deref, DerefMut, From};

use super::{eating::TryingToEat, movement::MovementSum};
use crate::{body, config, ecosystem, mind::Mind};

#[derive(Component, Debug, PartialEq, Eq, Deref, DerefMut, From, Add)]
pub struct BurntEnergy(ecosystem::Energy);

impl BurntEnergy {
    pub fn new() -> Self {
        BurntEnergy(ecosystem::Energy::new_empty())
    }
}

impl BurntEnergy {
    pub fn add_energy(&mut self, energy: ecosystem::Energy) {
        self.0 = self.0 + energy;
    }

    pub fn return_energy(&mut self) -> ecosystem::Energy {
        let amount = self.amount();
        self.take_energy(amount)
    }
}

pub fn burnt_energy_system(
    mut ecosystem: ResMut<ecosystem::Ecosystem>,
    mut burnt_query: Query<&mut BurntEnergy>,
) {
    for mut burnt_energy in burnt_query.iter_mut() {
        ecosystem.return_energy(burnt_energy.return_energy())
    }
}

pub fn thinking_energy_system(mut query: Query<(&Mind, &mut body::Vitality, &mut BurntEnergy)>) {
    for (bug_brain, mut vitality, mut burnt_energy) in query.iter_mut() {
        let thought_energy = vitality.take_energy(bug_brain.synapses().len());
        burnt_energy.add_energy(thought_energy);
    }
}

pub fn attempted_to_eat_system(
    time: Res<Time>,
    mut bug_query: Query<(&mut body::Vitality, &mut TryingToEat, &mut BurntEnergy)>,
) {
    for (mut vitality, mut trying_to_eat, mut burnt_energy) in bug_query.iter_mut() {
        trying_to_eat.tick(time.delta());
        if trying_to_eat.elapsed().as_secs_f32() >= 1.0 {
            burnt_energy
                .add_energy(vitality.take_energy(config::WorldConfig::global().eating_cost));
            trying_to_eat.reset()
        }
    }
}

pub fn movement_energy_burn_system(
    mut query: Query<(&mut body::Vitality, &mut MovementSum, &mut BurntEnergy)>,
) {
    for (mut vitality, mut movement_sum, mut burnt_energy) in query.iter_mut() {
        let energy = vitality.take_energy(movement_sum.uint_portion());
        burnt_energy.add_energy(energy)
    }
}
