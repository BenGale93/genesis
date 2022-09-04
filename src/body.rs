use std::time::Duration;

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

    pub fn proportion(&self) -> f64 {
        self.energy.as_uint() as f64 / self.energy_limit as f64
    }

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

    pub fn eat(&mut self, food_source: &mut food::Plant) -> Energy {
        let requested_energy = self.available_space();
        let extracted_energy = food_source.take_energy(requested_energy);
        self.add_energy(extracted_energy)
    }
}

#[derive(Component, Debug)]
pub struct EnergyStore {
    pub reserve: EnergyReserve,
}

#[derive(Component, Debug)]
pub struct Health {
    pub reserve: EnergyReserve,
}

#[derive(Component, Debug, PartialEq, Eq)]
pub struct CoreReserve {
    pub core: Energy,
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct Age(pub Stopwatch);

pub fn progress_age_system(time: Res<Time>, mut query: Query<&mut Age>) {
    for mut age in query.iter_mut() {
        age.0.tick(time.delta());
    }
}

#[derive(Bundle, Debug)]
pub struct BodyBundle {
    body: BugBody,
    energy_store: EnergyStore,
    health: Health,
    core_reserve: CoreReserve,
    age: Age,
}

impl BodyBundle {
    pub fn random(rng: &mut dyn RngCore, total_energy: Energy) -> Self {
        let body = BugBody::random(rng);

        Self::new(body, total_energy)
    }

    pub fn new(body: BugBody, total_energy: Energy) -> Self {
        let energy_split = total_energy.split(3);
        let energy_store = EnergyStore {
            reserve: EnergyReserve::new(energy_split[0]),
        };
        let health = Health {
            reserve: EnergyReserve::new(energy_split[1]),
        };
        let core_reserve = CoreReserve {
            core: energy_split[2],
        };

        Self {
            body,
            energy_store,
            health,
            core_reserve,
            age: Age(Stopwatch::new()),
        }
    }

    pub fn make_adult(mut self) -> Self {
        self.age
            .0
            .tick(Duration::from_secs_f32(self.body.adult_age_seconds()));
        self
    }
}
