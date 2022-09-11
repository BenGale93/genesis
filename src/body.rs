use std::{fmt, time::Duration};

use bevy::{prelude::*, time::Stopwatch};
use genesis_genome::Genome;
use genesis_util::Probability;
use rand::RngCore;

use crate::{config, ecosystem::Energy, food};

#[derive(Component, Debug, PartialEq, Eq)]
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

    pub fn mutate(&self, rng: &mut dyn RngCore) -> Self {
        let probability = Probability::new(
            self.genome
                .read_float(0.0, 0.2, 3, 0, 100)
                .expect(GENOME_READ_ERROR) as f64,
        )
        .expect("Expected to be between 0.0 and 1.0.");

        Self {
            genome: self.genome.mutate(rng, probability),
        }
    }

    pub fn rotate_speed(&self) -> f32 {
        self.genome
            .read_float(0.5, 3.0, 0, 0, 20)
            .expect(GENOME_READ_ERROR)
    }

    pub fn movement_speed(&self) -> f32 {
        self.genome
            .read_float(10.0, 100.0, 0, 0, 100)
            .expect(GENOME_READ_ERROR)
    }

    pub fn adult_age_seconds(&self) -> f32 {
        self.genome
            .read_float(30.0, 50.0, 1, 10, 10)
            .expect(GENOME_READ_ERROR)
    }

    pub fn death_age_seconds(&self) -> f32 {
        self.genome
            .read_float(600.0, 700.0, 1, 10, 10)
            .expect(GENOME_READ_ERROR)
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
    pub fn new(energy: Energy) -> Self {
        Self {
            energy,
            energy_limit: energy.as_uint(),
        }
    }

    pub fn amount(&self) -> usize {
        self.energy.as_uint()
    }

    #[must_use]
    pub fn proportion(&self) -> f64 {
        self.energy.as_uint() as f64 / self.energy_limit as f64
    }

    #[must_use]
    pub fn available_space(&self) -> usize {
        self.energy_limit - self.energy.as_uint()
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
    pub fn eat(&mut self, food_source: &mut food::Plant) -> Energy {
        let requested_energy = self.available_space();
        let extracted_energy = food_source.take_energy(requested_energy);
        self.add_energy(extracted_energy)
    }
}
impl fmt::Display for EnergyReserve {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.energy.as_uint(), self.energy_limit)
    }
}

#[derive(Debug)]
struct EnergyStore(EnergyReserve);

#[derive(Debug)]
struct Health(EnergyReserve);

#[derive(Debug, PartialEq, Eq)]
struct CoreReserve(Energy);

#[derive(Component, Debug)]
pub struct Vitality {
    energy_store: EnergyStore,
    health: Health,
    core_reserve: CoreReserve,
}

impl Vitality {
    pub fn new(_body: &BugBody, total_energy: Energy) -> Self {
        let energy_split = total_energy.split(3);
        let energy_store = EnergyStore(EnergyReserve::new(energy_split[0]));
        let health = Health(EnergyReserve::new(energy_split[1]));
        let core_reserve = CoreReserve(energy_split[2]);
        Self {
            energy_store,
            health,
            core_reserve,
        }
    }

    pub fn energy_store(&self) -> &EnergyReserve {
        &self.energy_store.0
    }

    pub fn health(&self) -> &EnergyReserve {
        &self.health.0
    }

    #[must_use]
    pub fn available_space(&self) -> usize {
        self.health().available_space() + self.energy_store().available_space()
    }

    #[must_use]
    pub fn add_energy(&mut self, energy: Energy) -> Energy {
        let remaining_energy = self.health.0.add_energy(energy);
        self.energy_store.0.add_energy(remaining_energy)
    }

    #[must_use]
    pub fn take_energy(&mut self, amount: usize) -> Energy {
        let mut taken_energy = self.energy_store.0.take_energy(amount);
        let still_needed = amount - taken_energy.as_uint();
        if still_needed > 0 {
            taken_energy = taken_energy + self.health.0.take_energy(still_needed);
        }
        taken_energy
    }

    #[must_use]
    pub fn eat(&mut self, food_source: &mut food::Plant) -> Energy {
        let requested_energy = self.available_space();
        let extracted_energy = food_source.take_energy(requested_energy);
        self.add_energy(extracted_energy)
    }
}

#[derive(Component, Debug)]
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
        let amount = self.0.as_uint();
        self.0.take_energy(amount)
    }
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct Age(pub Stopwatch);

impl fmt::Display for Age {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.elapsed().as_secs())
    }
}

pub fn progress_age_system(time: Res<Time>, mut query: Query<&mut Age>) {
    for mut age in query.iter_mut() {
        age.0.tick(time.delta());
    }
}

#[derive(Bundle, Debug)]
pub struct BodyBundle {
    body: BugBody,
    vitality: Vitality,
    age: Age,
    burnt_energy: BurntEnergy,
}

impl BodyBundle {
    pub fn random(rng: &mut dyn RngCore, total_energy: Energy) -> Self {
        let body = BugBody::random(rng);

        Self::new(body, total_energy)
    }

    pub fn new(body: BugBody, total_energy: Energy) -> Self {
        let vitality = Vitality::new(&body, total_energy);
        Self {
            body,
            vitality,
            age: Age(Stopwatch::new()),
            burnt_energy: BurntEnergy::new(),
        }
    }

    pub fn make_adult(mut self) -> Self {
        self.age
            .0
            .tick(Duration::from_secs_f32(self.body.adult_age_seconds()));
        self
    }
}
