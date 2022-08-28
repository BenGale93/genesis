extern crate derive_more;
use derive_more::{Add, Display, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Add, Display, Sub)]
pub struct Energy(u64);

impl Energy {
    fn new(e: u64) -> Self {
        Self(e)
    }

    pub fn as_uint(&self) -> u64 {
        self.0
    }

    pub fn split(self, divisor: u64) -> Vec<Self> {
        let mut output = Vec::new();
        let mut starting_energy = self.0;
        for _ in 0..divisor {
            let new_energy = self.0 / divisor;
            starting_energy -= new_energy;
            output.push(new_energy);
        }

        for i in 0..starting_energy {
            output[i as usize] += 1;
        }

        output.iter().map(|&e| Energy::new(e)).collect()
    }
}

pub struct Ecosystem {
    energy: Energy,
}

impl Ecosystem {
    pub fn new(energy: u64) -> Self {
        Self {
            energy: Energy(energy),
        }
    }

    pub fn available_energy(&self) -> Energy {
        self.energy
    }

    pub fn request_energy(&mut self, units: u64) -> Option<Energy> {
        let requested_energy = Energy(units);
        if requested_energy > self.energy {
            None
        } else {
            self.energy = self.energy - requested_energy;
            Some(requested_energy)
        }
    }

    pub fn return_energy(&mut self, energy: Energy) {
        self.energy = self.energy + energy;
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::ecosystem;

    #[test]
    fn request_energy_success() {
        let mut eco_system = ecosystem::Ecosystem::new(100);

        let energy = eco_system.request_energy(20).unwrap();

        assert_eq!(energy.as_uint(), 20);
        assert_eq!(eco_system.available_energy().as_uint(), 80);
    }

    #[rstest]
    #[case((99,3), vec![33,33,33])]
    #[case((100,3), vec![34,33,33])]
    #[case((101,3), vec![34,34,33])]
    #[case((101,4), vec![26,25,25,25])]
    fn split_doesnt_create_new_energy(#[case] inputs: (u64, u64), #[case] expected: Vec<u64>) {
        let energy = ecosystem::Energy::new(inputs.0);

        let split_energy = energy.split(inputs.1);

        for (exp, e) in expected.iter().zip(split_energy.iter()) {
            assert_eq!(&e.as_uint(), exp);
        }
    }
}
