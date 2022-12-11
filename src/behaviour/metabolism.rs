use bevy::prelude::{Component, Query, ResMut};
use derive_more::{Add, Deref, DerefMut, From};

use super::{eating::EatingSum, growth::SizeSum, movement::MovementSum, thinking::ThinkingSum};
use crate::{body, ecosystem};

#[derive(Component, Debug, PartialEq, Eq, Deref, DerefMut, From, Add)]
pub struct BurntEnergy(ecosystem::Energy);

impl BurntEnergy {
    pub const fn new() -> Self {
        Self(ecosystem::Energy::new_empty())
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
        macro_rules! shift_energy {
            ($energy_sum:ident) => {
                let cost = $energy_sum.uint_portion();
                if cost >= 1 {
                    burnt_energy.add_energy(vitality.take_energy(cost));
                }
            };
        }
        shift_energy!(thinking_sum);
        shift_energy!(eating_sum);
        shift_energy!(movement_sum);
        shift_energy!(size_sum);

        ecosystem.return_energy(burnt_energy.return_energy())
    }
}
