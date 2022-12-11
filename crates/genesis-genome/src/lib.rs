#![warn(clippy::all, clippy::nursery)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::similar_names)]
#![allow(clippy::many_single_char_names)]
mod chromosome;
mod genome_error;

use bitvec::slice::BitSlice;
use chromosome::Chromosome;
use genesis_util::Probability;
pub use genome_error::GenomeError;
use ndarray::Array;
use rand::RngCore;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Genome {
    chromosomes: Vec<Chromosome>,
}

impl Genome {
    #[must_use]
    pub fn new(number: usize, length: usize) -> Self {
        let mut chromosomes = Vec::with_capacity(number);

        for _ in 0..number {
            chromosomes.push(Chromosome::new(length));
        }
        Self { chromosomes }
    }

    pub fn random(rng: &mut dyn RngCore, number: usize, length: usize) -> Self {
        let mut chromosomes = Vec::with_capacity(number);

        for _ in 0..number {
            chromosomes.push(Chromosome::random(rng, length));
        }
        Self { chromosomes }
    }

    #[must_use]
    pub fn mutate(&self, rng: &mut dyn RngCore, chance: Probability) -> Self {
        let mut new_chromosomes = Vec::with_capacity(self.chromosomes.len());

        for c in &self.chromosomes {
            new_chromosomes.push(c.mutate(rng, chance));
        }
        Self {
            chromosomes: new_chromosomes,
        }
    }

    pub fn read(
        &self,
        location: usize,
        start: usize,
        length: usize,
    ) -> Result<&BitSlice<u8>, GenomeError> {
        let c = self
            .chromosomes
            .get(location)
            .ok_or(GenomeError::ChromosomeNotFoundError)?;

        c.read(start, length)
    }

    pub fn read_float(
        &self,
        min: f32,
        max: f32,
        location: usize,
        start: usize,
        length: usize,
    ) -> Result<f32, GenomeError> {
        let dna = self.read(location, start, length)?;

        let count = dna.count_ones();

        let array = Array::linspace(min, max, dna.len() + 1);

        Ok(*array
            .get(count)
            .expect("count should be strictly less than len."))
    }
}

#[cfg(test)]
mod tests {
    use bitvec::prelude::*;
    use genesis_util::Probability;
    use rand::{prelude::StdRng, SeedableRng};

    use super::Genome;

    #[test]
    fn test_mutate_changes() {
        let mut rng = StdRng::seed_from_u64(1);
        let parent = Genome::random(&mut rng, 2, 10);
        let child = parent.mutate(&mut rng, Probability::new(0.5).unwrap());
        assert_ne!(parent, child);
    }

    #[test]
    fn read_from_chromosome_via_genome() {
        let g = Genome::new(2, 10);

        assert_eq!(g.read(0, 1, 4).unwrap(), &bits![0;4]);
    }

    #[test]
    #[should_panic(expected = "value: ChromosomeNotFoundError")]
    fn read_error_from_genome() {
        let c = Genome::new(1, 10);

        c.read(2, 0, 4).unwrap();
    }
}
