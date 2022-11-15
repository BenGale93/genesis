use bevy::prelude::{Component, Query, ResMut};
use derive_more::{Add, Deref, DerefMut, From};

use super::{eating::EatingSum, growth::SizeSum, movement::MovementSum, thinking::ThinkingSum};
use crate::{body, ecosystem};

#[derive(Component, Debug, PartialEq, Eq, Deref, DerefMut, From, Add)]
pub struct BurntEnergy(ecosystem::Energy);

impl BurntEnergy {
    pub fn new() -> Self {
        BurntEnergy(ecosystem::Energy::new_empty())
    }
}

impl BurntEnergy {
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

pub fn thinking_energy_system(
    mut query: Query<(&mut body::Vitality, &mut ThinkingSum, &mut BurntEnergy)>,
) {
    for (mut vitality, mut thoughts, mut burnt_energy) in query.iter_mut() {
        let thought_cost = thoughts.uint_portion();
        if thought_cost >= 1 {
            burnt_energy.add_energy(vitality.take_energy(thought_cost));
        }
    }
}

pub fn eating_energy_system(
    mut query: Query<(&mut body::Vitality, &mut EatingSum, &mut BurntEnergy)>,
) {
    for (mut vitality, mut eating_sum, mut burnt_energy) in query.iter_mut() {
        let eating_cost = eating_sum.uint_portion();
        if eating_cost >= 1 {
            burnt_energy.add_energy(vitality.take_energy(eating_cost));
        }
    }
}

pub fn movement_energy_burn_system(
    mut query: Query<(&mut body::Vitality, &mut MovementSum, &mut BurntEnergy)>,
) {
    for (mut vitality, mut movement_sum, mut burnt_energy) in query.iter_mut() {
        let movement_cost = movement_sum.uint_portion();
        if movement_cost >= 1 {
            burnt_energy.add_energy(vitality.take_energy(movement_cost));
        }
    }
}

pub fn size_energy_system(mut query: Query<(&mut body::Vitality, &mut SizeSum, &mut BurntEnergy)>) {
    for (mut vitality, mut size_sum, mut burnt_energy) in query.iter_mut() {
        let size_cost = size_sum.uint_portion();
        if size_cost >= 1 {
            burnt_energy.add_energy(vitality.take_energy(size_cost));
        }
    }
}
