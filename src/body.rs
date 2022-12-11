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

    pub const fn genome(&self) -> &Genome {
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

    pub const fn amount(&self) -> usize {
        self.energy.amount()
    }

    #[must_use]
    pub fn proportion(&self) -> f32 {
        self.energy.amount() as f32 / self.energy_limit as f32
    }

    #[must_use]
    const fn available_space(&self) -> usize {
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

    pub const fn energy_limit(&self) -> usize {
        self.energy_limit
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
    size: Size,
    energy_store: EnergyStore,
    health: Health,
    core_reserve: CoreReserve,
}

impl Vitality {
    pub fn new(size: Size, mut total_energy: ecosystem::Energy) -> (Self, ecosystem::Energy) {
        let size_uint = size.as_uint();
        let core_energy = total_energy.take_energy(config::CORE_MULTIPLIER * size_uint);
        let core_reserve = CoreReserve(core_energy);

        let health_energy = total_energy.take_energy(config::HEALTH_MULTIPLIER * size_uint);
        let health = Health(
            EnergyReserve::new(health_energy, config::HEALTH_MULTIPLIER * size_uint).unwrap(),
        );

        let energy_limit = config::EnergyLimitConfig::global().energy_limit(size.as_uint());
        let energy_store = EnergyStore(
            EnergyReserve::new(total_energy.take_energy(energy_limit), energy_limit).unwrap(),
        );

        (
            Self {
                size,
                energy_store,
                health,
                core_reserve,
            },
            total_energy,
        )
    }

    pub fn energy_store(&self) -> &EnergyReserve {
        &self.energy_store
    }

    pub fn health(&self) -> &EnergyReserve {
        &self.health
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
        let requested_energy = self
            .available_space()
            .min(self.size.as_uint() / config::EATING_RATIO);
        let extracted_energy = plant.take_energy(requested_energy);
        self.add_energy(extracted_energy)
    }

    pub const fn size(&self) -> &Size {
        &self.size
    }

    pub fn grow(&mut self, amount: usize) -> Result<()> {
        if self.size.at_max_size()
            || (self.energy_store().amount()
                < amount * (config::CORE_MULTIPLIER + config::HEALTH_MULTIPLIER))
        {
            return Err(anyhow!("Can't grow."));
        }
        let core_growing_energy = self
            .energy_store
            .take_energy(amount * config::CORE_MULTIPLIER);
        self.core_reserve.add_energy(core_growing_energy);

        let health_growing_energy = self
            .energy_store
            .take_energy(amount * config::HEALTH_MULTIPLIER);
        self.health.energy_limit += amount * config::HEALTH_MULTIPLIER;

        if self.health.add_energy(health_growing_energy) != ecosystem::Energy::new_empty() {
            panic!("Tried to grow and couldn't add all the energy to health.")
        };

        self.size.grow(amount as f32);

        self.energy_store.energy_limit =
            config::EnergyLimitConfig::global().energy_limit(self.size.as_uint());
        Ok(())
    }

    #[must_use]
    pub fn take_all_energy(&mut self) -> ecosystem::Energy {
        let mut returning_energy = self.energy_store.0.energy.take_all_energy();
        returning_energy = returning_energy + self.health.0.energy.take_all_energy();
        returning_energy = returning_energy + self.core_reserve.0.take_all_energy();
        returning_energy
    }
}

#[derive(Debug)]
pub struct Size {
    current_size: f32,
    max_size: f32,
}

impl Size {
    pub const fn new(size: f32, max_size: f32) -> Self {
        Self {
            current_size: size,
            max_size,
        }
    }

    pub const fn current_size(&self) -> f32 {
        self.current_size
    }

    pub fn grow(&mut self, increment: f32) {
        self.current_size = (self.current_size + increment).min(self.max_size);
    }

    pub const fn sprite(&self) -> Vec2 {
        Vec2::splat(self.current_size)
    }

    pub fn collider(&self) -> Collider {
        Collider::capsule(
            Vec2::new(0.0, -self.current_size / 5.5),
            Vec2::new(0.0, self.current_size / 5.5),
            self.current_size / 3.5,
        )
    }

    pub const fn as_uint(&self) -> usize {
        self.current_size as usize
    }

    pub fn at_max_size(&self) -> bool {
        self.current_size == self.max_size
    }
}
