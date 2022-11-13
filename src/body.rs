use std::fmt;

use anyhow::{anyhow, Result};
use bevy::prelude::{Color, Component, Vec2};
use bevy_rapier2d::prelude::Collider;
use derive_more::{Deref, DerefMut};
use genesis_genome::Genome;
use genesis_util::Probability;
use rand::RngCore;

use crate::{config, ecosystem};

#[derive(Component, Debug, Deref, DerefMut)]
pub struct OriginalColor(pub Color);

#[derive(Component, Debug, PartialEq, Eq, Clone)]
pub struct BugBody {
    genome: Genome,
}

impl BugBody {
    fn new() -> Self {
        let genome = Genome::new(config::CHROMOSOME_COUNT, config::CHROMOSOME_LEN);

        Self { genome }
    }

    pub fn random(rng: &mut dyn RngCore) -> Self {
        let genome = Genome::random(rng, config::CHROMOSOME_COUNT, config::CHROMOSOME_LEN);

        Self { genome }
    }

    pub fn mutate(&self, rng: &mut dyn RngCore, probability: Probability) -> Self {
        Self {
            genome: self.genome.mutate(rng, probability),
        }
    }

    pub fn genome(&self) -> &Genome {
        &self.genome
    }
}

impl Default for BugBody {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct EnergyReserve {
    energy: ecosystem::Energy,
    energy_limit: usize,
}

impl EnergyReserve {
    fn new(energy: ecosystem::Energy, limit: usize) -> Result<Self> {
        if energy.amount() > limit {
            return Err(anyhow!("Limit should be higher than energy passed in."));
        }
        Ok(Self {
            energy,
            energy_limit: limit,
        })
    }

    pub fn amount(&self) -> usize {
        self.energy.amount()
    }

    #[must_use]
    pub fn proportion(&self) -> f64 {
        self.energy.amount() as f64 / self.energy_limit as f64
    }

    #[must_use]
    fn available_space(&self) -> usize {
        self.energy_limit - self.energy.amount()
    }

    #[must_use]
    fn add_energy(&mut self, mut energy: ecosystem::Energy) -> ecosystem::Energy {
        let energy_taken = energy.take_energy(self.available_space());
        self.energy.add_energy(energy_taken);
        energy
    }

    #[must_use]
    fn take_energy(&mut self, amount: usize) -> ecosystem::Energy {
        self.energy.take_energy(amount)
    }
}
impl fmt::Display for EnergyReserve {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.energy.amount(), self.energy_limit)
    }
}

#[derive(Debug, Deref, DerefMut)]
struct EnergyStore(EnergyReserve);

#[derive(Debug, Deref, DerefMut)]
struct Health(EnergyReserve);

#[derive(Debug, PartialEq, Eq, Deref, DerefMut)]
pub struct CoreReserve(ecosystem::Energy);

#[derive(Component, Debug)]
pub struct Vitality {
    energy_store: EnergyStore,
    health: Health,
    core_reserve: CoreReserve,
}

const CORE_MULTIPLIER: usize = 2;
const HEALTH_MULTIPLIER: usize = 3;
const ENERGY_MULTIPLIER: usize = 7;

impl Vitality {
    pub fn new(size: usize, mut total_energy: ecosystem::Energy) -> Self {
        let core_energy = total_energy.take_energy(CORE_MULTIPLIER * size);
        let core_reserve = CoreReserve(core_energy);

        let health_energy = total_energy.take_energy(HEALTH_MULTIPLIER * size);
        let health = Health(EnergyReserve::new(health_energy, HEALTH_MULTIPLIER * size).unwrap());

        let energy_limit = if total_energy.amount() < ENERGY_MULTIPLIER * size {
            ENERGY_MULTIPLIER * size
        } else {
            total_energy.amount()
        };
        let energy_store = EnergyStore(EnergyReserve::new(total_energy, energy_limit).unwrap());
        Self {
            energy_store,
            health,
            core_reserve,
        }
    }

    pub fn energy_store(&self) -> &EnergyReserve {
        &self.energy_store
    }

    pub fn health(&self) -> &EnergyReserve {
        &self.health
    }

    #[allow(dead_code)]
    pub fn core_reserve(&self) -> &CoreReserve {
        &self.core_reserve
    }

    #[must_use]
    pub fn available_space(&self) -> usize {
        self.health().available_space() + self.energy_store().available_space()
    }

    #[must_use]
    pub fn add_energy(&mut self, energy: ecosystem::Energy) -> ecosystem::Energy {
        let remaining_energy = self.health.add_energy(energy);
        self.energy_store.add_energy(remaining_energy)
    }

    #[must_use]
    pub fn take_energy(&mut self, amount: usize) -> ecosystem::Energy {
        let mut taken_energy = self.energy_store.take_energy(amount);
        let still_needed = amount - taken_energy.amount();
        if still_needed > 0 {
            taken_energy = taken_energy + self.health.take_energy(still_needed);
        }
        taken_energy
    }

    #[must_use]
    pub fn eat(&mut self, plant: &mut ecosystem::Plant) -> ecosystem::Energy {
        let requested_energy = self.available_space();
        let extracted_energy = plant.take_energy(requested_energy);
        self.add_energy(extracted_energy)
    }
}

#[derive(Debug, Component, DerefMut, Deref)]
pub struct Size(pub f32);

impl Size {
    pub fn sprite(&self) -> Vec2 {
        Vec2::splat(self.0)
    }

    pub fn collider(&self) -> Collider {
        Collider::capsule(
            Vec2::new(0.0, -self.0 / 5.5),
            Vec2::new(0.0, self.0 / 5.5),
            self.0 / 3.5,
        )
    }
}
