use bevy::prelude::*;

use crate::ecosystem::Energy;

#[derive(Component, Debug)]
pub struct Plant {
    energy: Energy,
}

impl Plant {
    pub fn new(energy: Energy) -> Self {
        Self { energy }
    }

    pub fn take_energy(&mut self, amount: usize) -> Energy {
        self.energy.take_energy(amount)
    }

    pub fn energy(&self) -> Energy {
        self.energy
    }
}
