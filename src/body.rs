use bevy::prelude::*;
use genesis_genome::Genome;
use genesis_util::Probability;
use rand::RngCore;

use crate::{config, ecosystem::Energy};

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

#[derive(Bundle, Debug)]
pub struct BodyBundle {
    body: BugBody,
    energy_store: EnergyStore,
    health: Health,
    core_reserve: CoreReserve,
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
        }
    }
}
