use bevy::prelude::*;
use genesis_genome::Genome;
use genesis_util::Probability;
use rand::RngCore;

use crate::config;

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
