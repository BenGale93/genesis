extern crate derive_more;
use bevy::prelude::Component;
use derive_more::{Add, Constructor, Display, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Add, Display, Sub)]
pub struct Energy(usize);

impl Energy {
    fn new(e: usize) -> Self {
        Self(e)
    }

    pub fn amount(&self) -> usize {
        self.0
    }

    #[must_use]
    pub fn take_energy(&mut self, amount: usize) -> Self {
        let to_return = amount.min(self.0);
        self.0 -= to_return;
        Energy::new(to_return)
    }

    pub fn new_empty() -> Self {
        Self(0)
    }
}

#[derive(Component, Debug, Constructor)]
pub struct Plant {
    energy: Energy,
}

impl Plant {
    pub fn take_energy(&mut self, amount: usize) -> Energy {
        self.energy.take_energy(amount)
    }

    pub fn energy(&self) -> Energy {
        self.energy
    }
}

#[derive(Debug)]
pub struct Ecosystem {
    energy: Energy,
}

impl Ecosystem {
    pub fn new(energy: usize) -> Self {
        Self {
            energy: Energy(energy),
        }
    }

    pub fn available_energy(&self) -> Energy {
        self.energy
    }

    pub fn request_energy(&mut self, units: usize) -> Option<Energy> {
        let requested_energy = Energy(units);
        if requested_energy > self.energy {
            None
        } else {
            self.energy = self.energy - requested_energy;
            Some(requested_energy)
        }
    }

    pub fn return_energy(&mut self, energy: Energy) {
        self.energy = self.energy + energy;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{body::Vitality, config, ecosystem};

    #[test]
    fn request_energy_success() {
        let mut eco_system = ecosystem::Ecosystem::new(100);

        let energy = eco_system.request_energy(20).unwrap();

        assert_eq!(energy.amount(), 20);
        assert_eq!(eco_system.available_energy().amount(), 80);
    }

    #[test]
    fn move_all_energy_empties_vitality() {
        config::initialize_config();
        let initial_energy = Energy::new(1000);
        let mut vitality = Vitality::new(initial_energy);

        let moved_energy = vitality.move_all_energy();

        assert_eq!(vitality.health().amount(), 0);
        assert_eq!(vitality.energy_store().amount(), 0);
        assert_eq!(vitality.core_reserve().amount(), 0);
        assert_eq!(moved_energy.amount(), 1000);
    }
}
