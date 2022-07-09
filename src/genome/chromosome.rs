use bitvec::prelude::*;
use rand::{Rng, RngCore};

use crate::{genome::errors::GenomeError, probability::Probability};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chromosome {
    dna: BitVec<u8>,
}

impl Chromosome {
    pub fn new(length: usize) -> Self {
        Self {
            dna: bitvec![u8, Lsb0; 0; length],
        }
    }

    pub fn len(&self) -> usize {
        self.dna.len()
    }

    pub fn random(rng: &mut dyn RngCore, length: usize) -> Self {
        let mut bv = BitVec::with_capacity(length);
        for _ in 0..length {
            if rng.gen_bool(0.5) {
                bv.push(true);
            } else {
                bv.push(false);
            }
        }
        Self { dna: bv }
    }

    pub fn mutate(&self, rng: &mut dyn RngCore, chance: Probability) -> Self {
        let mut bv = BitVec::with_capacity(self.len());
        for bit in self.dna.iter().by_vals() {
            if rng.gen_bool(chance.as_float()) {
                bv.push(!bit);
            } else {
                bv.push(bit);
            }
        }
        Self { dna: bv }
    }

    pub fn read(&self, start: usize, length: usize) -> Result<&BitSlice<u8>, GenomeError> {
        let end = start + length;
        let range = start..end;

        self.dna.get(range).ok_or(GenomeError::DnaReadError)
    }
}

#[cfg(test)]
mod tests {
    use bitvec::prelude::*;
    use rand::{prelude::StdRng, SeedableRng};

    use super::Chromosome;
    use crate::probability::Probability;

    #[test]
    fn test_mutate_changes() {
        let mut rng = StdRng::seed_from_u64(1);
        let parent = Chromosome::random(&mut rng, 10);
        let child = parent.mutate(&mut rng, Probability::new(0.5).unwrap());
        assert_ne!(parent, child);
    }

    #[test]
    fn read_from_chromosome() {
        let c = Chromosome::new(10);

        assert_eq!(c.read(1, 4).unwrap(), &bits![0;4]);
    }

    #[test]
    #[should_panic(expected = "value: DnaReadError")]
    fn read_error_from_chromosome() {
        let c = Chromosome::new(10);

        c.read(9, 4).unwrap();
    }
}
