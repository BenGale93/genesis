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

pub fn energy_return_system(
    mut ecosystem: ResMut<ecosystem::Ecosystem>,
    mut query: Query<(
        &mut body::Vitality,
        &mut ThinkingSum,
        &mut EatingSum,
        &mut MovementSum,
        &mut SizeSum,
        &mut BurntEnergy,
    )>,
) {
    for (
        mut vitality,
        mut thinking_sum,
        mut eating_sum,
        mut movement_sum,
        mut size_sum,
        mut burnt_energy,
    ) in query.iter_mut()
    {
        let thought_cost = thinking_sum.uint_portion();
        if thought_cost >= 1 {
            burnt_energy.add_energy(vitality.take_energy(thought_cost));
        }

        let eating_cost = eating_sum.uint_portion();
        if eating_cost >= 1 {
            burnt_energy.add_energy(vitality.take_energy(eating_cost));
        }

        let movement_cost = movement_sum.uint_portion();
        if movement_cost >= 1 {
            burnt_energy.add_energy(vitality.take_energy(movement_cost));
        }
        let size_cost = size_sum.uint_portion();
        if size_cost >= 1 {
            burnt_energy.add_energy(vitality.take_energy(size_cost));
        }

        ecosystem.return_energy(burnt_energy.return_energy())
    }
}
