#![warn(clippy::all, clippy::nursery)]
extern crate derive_more;
use std::fmt;

use anyhow::{anyhow, Result};
use bevy_app::Plugin;
use bevy_ecs::{
    prelude::{Component, Resource},
    reflect::ReflectComponent,
};
use bevy_reflect::Reflect;
use derive_more::{Add, Constructor, Display, Sub};
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Add,
    Display,
    Sub,
    Default,
    Reflect,
    Serialize,
    Deserialize,
    Clone,
)]
pub struct Energy(usize);

impl Energy {
    const fn new(e: usize) -> Self {
        Self(e)
    }

    pub const fn amount(&self) -> usize {
        self.0
    }

    #[must_use]
    pub fn take_energy(&mut self, amount: usize) -> Self {
        let to_return = amount.min(self.0);
        self.0 -= to_return;
        Self::new(to_return)
    }

    pub fn add_energy(&mut self, energy: Self) {
        self.0 += energy.0;
    }

    pub const fn new_empty() -> Self {
        Self(0)
    }

    #[must_use]
    pub fn take_all_energy(&mut self) -> Self {
        self.take_energy(self.amount())
    }
}

#[derive(Component, Debug, Constructor, Default, Reflect)]
#[reflect(Component)]
pub struct Food {
    energy: Energy,
    energy_density: usize,
    toughness: f32,
}

impl Food {
    pub const fn energy(&self) -> &Energy {
        &self.energy
    }

    pub const fn energy_density(&self) -> usize {
        self.energy_density
    }

    pub const fn toughness(&self) -> f32 {
        self.toughness
    }

    pub fn take_energy(&mut self, amount: usize) -> Energy {
        self.energy.take_energy(amount)
    }

    pub const fn size(&self) -> f32 {
        (self.energy.amount() / self.energy_density) as f32
    }

    pub fn add_energy(&mut self, energy: Energy) {
        self.energy.add_energy(energy);
    }
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct EggEnergy(pub Energy);

impl EggEnergy {
    #[must_use]
    pub fn move_all_energy(&mut self) -> Energy {
        self.0.take_energy(self.0.amount())
    }
}

#[derive(Debug, Resource, Serialize, Deserialize, Clone)]
pub struct Ecosystem {
    energy: Energy,
}

impl Ecosystem {
    pub const fn new(energy: usize) -> Self {
        Self {
            energy: Energy(energy),
        }
    }

    pub const fn available_energy(&self) -> &Energy {
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

#[derive(Debug, Default, Reflect)]
pub struct EnergyReserve {
    energy: Energy,
    energy_limit: usize,
}

impl EnergyReserve {
    pub fn new(energy: Energy, limit: usize) -> Result<Self> {
        if energy.amount() > limit {
            return Err(anyhow!("Limit should be higher than energy passed in."));
        }
        Ok(Self {
            energy,
            energy_limit: limit,
        })
    }

    pub const fn amount(&self) -> usize {
        self.energy.amount()
    }

    #[must_use]
    pub fn proportion(&self) -> f32 {
        self.energy.amount() as f32 / self.energy_limit as f32
    }

    #[must_use]
    pub const fn available_space(&self) -> usize {
        self.energy_limit - self.energy.amount()
    }

    #[must_use]
    pub fn add_energy(&mut self, mut energy: Energy) -> Energy {
        let energy_taken = energy.take_energy(self.available_space());
        self.energy.add_energy(energy_taken);
        energy
    }

    #[must_use]
    pub fn take_energy(&mut self, amount: usize) -> Energy {
        self.energy.take_energy(amount)
    }

    pub const fn energy_limit(&self) -> usize {
        self.energy_limit
    }

    pub fn set_energy_limit(&mut self, energy_limit: usize) {
        self.energy_limit = energy_limit;
    }

    pub fn take_all_energy(&mut self) -> Energy {
        self.energy.take_all_energy()
    }
}
impl fmt::Display for EnergyReserve {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.energy.amount(), self.energy_limit)
    }
}

pub struct EcosystemPlugin;

impl Plugin for EcosystemPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<Energy>()
            .register_type::<EggEnergy>()
            .register_type::<Food>()
            .register_type::<EnergyReserve>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_energy_success() {
        let mut eco_system = Ecosystem::new(100);

        let energy = eco_system.request_energy(20).unwrap();

        assert_eq!(energy.amount(), 20);
        assert_eq!(eco_system.available_energy().amount(), 80);
    }
}
