use std::{fmt, time::Duration};

use anyhow::{anyhow, Result};
use bevy::{prelude::*, time::Stopwatch};
use derive_more::{Add, From};
use genesis_genome::Genome;
use genesis_util::Probability;
use rand::RngCore;

use crate::{
    config,
    ecosystem::{Energy, Plant},
};

#[derive(Component, Debug, PartialEq, Eq, Clone)]
pub struct BugBody {
    genome: Genome,
}

const GENOME_READ_ERROR: &str = "Expected to be able to read from here";

impl BugBody {
    pub fn new() -> Self {
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

    pub fn amount(&self) -> usize {
        self.energy.amount()
    }

    #[must_use]
    pub fn proportion(&self) -> f64 {
        self.energy.amount() as f64 / self.energy_limit as f64
    }

    #[must_use]
    pub fn available_space(&self) -> usize {
        self.energy_limit - self.energy.amount()
    }

    #[must_use]
    pub fn add_energy(&mut self, mut energy: Energy) -> Energy {
        let energy_taken = energy.take_energy(self.available_space());
        self.energy = self.energy + energy_taken;
        energy
    }

    #[must_use]
    pub fn take_energy(&mut self, amount: usize) -> Energy {
        self.energy.take_energy(amount)
    }

    #[must_use]
    pub fn eat(&mut self, plant: &mut Plant) -> Energy {
        let requested_energy = self.available_space();
        let extracted_energy = plant.take_energy(requested_energy);
        self.add_energy(extracted_energy)
    }

    #[must_use]
    pub fn take_all_energy(&mut self) -> Energy {
        let amount = self.amount();
        self.take_energy(amount)
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
pub struct CoreReserve(Energy);

#[derive(Component, Debug)]
pub struct Vitality {
    energy_store: EnergyStore,
    health: Health,
    core_reserve: CoreReserve,
}

impl Vitality {
    pub fn new(mut total_energy: Energy) -> Self {
        let core_energy = config::WorldConfig::global().core_energy;
        let core_reserve = CoreReserve(total_energy.take_energy(core_energy));

        let health_energy = config::WorldConfig::global().health_energy;
        let health = Health(
            EnergyReserve::new(total_energy.take_energy(health_energy), health_energy).unwrap(),
        );

        let energy_limit = config::WorldConfig::global().start_energy - core_energy - health_energy;
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

    pub fn core_reserve(&self) -> &CoreReserve {
        &self.core_reserve
    }

    #[must_use]
    pub fn available_space(&self) -> usize {
        self.health().available_space() + self.energy_store().available_space()
    }

    #[must_use]
    pub fn add_energy(&mut self, energy: Energy) -> Energy {
        let remaining_energy = self.health.add_energy(energy);
        self.energy_store.add_energy(remaining_energy)
    }

    #[must_use]
    pub fn take_energy(&mut self, amount: usize) -> Energy {
        let mut taken_energy = self.energy_store.take_energy(amount);
        let still_needed = amount - taken_energy.amount();
        if still_needed > 0 {
            taken_energy = taken_energy + self.health.take_energy(still_needed);
        }
        taken_energy
    }

    #[must_use]
    pub fn eat(&mut self, plant: &mut Plant) -> Energy {
        let requested_energy = self.available_space();
        let extracted_energy = plant.take_energy(requested_energy);
        self.add_energy(extracted_energy)
    }

    #[must_use]
    pub fn move_all_energy(&mut self) -> Energy {
        let core_energy = self.core_reserve.amount();
        let mut moved_energy = self.core_reserve.take_energy(core_energy);

        moved_energy = moved_energy + self.health.take_all_energy();
        moved_energy = moved_energy + self.energy_store.take_all_energy();
        moved_energy
    }
}

#[derive(Component, Debug, PartialEq, Eq, Deref, DerefMut, From, Add)]
pub struct BurntEnergy(Energy);

impl BurntEnergy {
    pub fn new() -> Self {
        BurntEnergy(Energy::new_empty())
    }
}

impl BurntEnergy {
    pub fn add_energy(&mut self, energy: Energy) {
        self.0 = self.0 + energy;
    }

    pub fn return_energy(&mut self) -> Energy {
        let amount = self.amount();
        self.take_energy(amount)
    }
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct Age(pub Stopwatch);

impl Age {
    pub fn new(seconds: f32) -> Self {
        let mut age = Age::default();
        age.tick(Duration::from_secs_f32(seconds));
        age
    }
}

impl Default for Age {
    fn default() -> Self {
        Self(Stopwatch::new())
    }
}

impl fmt::Display for Age {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.elapsed().as_secs())
    }
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct Heart(pub Stopwatch);

impl Heart {
    pub fn new() -> Self {
        Self(Stopwatch::new())
    }

    pub fn pulse(&self) -> f32 {
        self.elapsed_secs().sin()
    }
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct InternalTimer(pub Stopwatch);

impl InternalTimer {
    pub fn new() -> Self {
        Self(Stopwatch::new())
    }
}

impl Default for InternalTimer {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for InternalTimer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.elapsed().as_secs())
    }
}

pub fn progress_age_system(time: Res<Time>, mut query: Query<&mut Age>) {
    for mut age in query.iter_mut() {
        age.tick(time.delta());
    }
}

pub fn progress_timers_system(time: Res<Time>, mut query: Query<(&mut Heart, &mut InternalTimer)>) {
    for (mut heart, mut internal_timer) in query.iter_mut() {
        heart.tick(time.delta());
        internal_timer.tick(time.delta());
    }
}
