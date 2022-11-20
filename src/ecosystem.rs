extern crate derive_more;
use bevy::prelude::{Component, Resource};
use derive_more::{Add, Constructor, Display, Sub};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Add, Display, Sub)]
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

    pub fn add_energy(&mut self, energy: Energy) {
        self.0 += energy.0;
    }

    pub fn new_empty() -> Self {
        Self(0)
    }

    #[must_use]
    pub fn take_all_energy(&mut self) -> Self {
        self.take_energy(self.amount())
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

    pub fn energy(&self) -> &Energy {
        &self.energy
    }
}

#[derive(Debug, Resource)]
pub struct Ecosystem {
    energy: Energy,
}

impl Ecosystem {
    pub fn new(energy: usize) -> Self {
        Self {
            energy: Energy(energy),
        }
    }

    pub fn available_energy(&self) -> &Energy {
        &self.energy
    }

    pub fn request_energy(&mut self, units: usize) -> Option<Energy> {
        if units > self.energy.0 {
            None
        } else {
            let requested_energy = self.energy.take_energy(units);
            Some(requested_energy)
        }
    }

    pub fn return_energy(&mut self, energy: Energy) {
        self.energy.add_energy(energy);
    }
}

#[cfg(test)]
mod tests {
    use crate::ecosystem;

    #[test]
    fn request_energy_success() {
        let mut eco_system = ecosystem::Ecosystem::new(100);

        let energy = eco_system.request_energy(20).unwrap();

        assert_eq!(energy.amount(), 20);
        assert_eq!(eco_system.available_energy().amount(), 80);
    }
}
