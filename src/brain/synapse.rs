use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
};

use crate::{brain::BrainError, weight::Weight};

#[derive(Debug, Clone)]
pub struct Synapse {
    from: usize,
    to: usize,
    weight: Weight,
    active: bool,
    innovation: usize,
}

impl Synapse {
    pub fn new(from: usize, to: usize) -> Result<Self, BrainError> {
        if from == to {
            return Err(BrainError::InvalidFromTo);
        }
        let innovation = Synapse::compute_innovation(from, to);
        let weight = Weight::random();

        Ok(Synapse {
            from,
            to,
            weight,
            active: true,
            innovation,
        })
    }

    pub fn with_weight(from: usize, to: usize, weight: Weight) -> Result<Self, BrainError> {
        let mut synapse = Synapse::new(from, to)?;
        synapse.set_weight(weight);
        Ok(synapse)
    }

    // Cantor Pairing Function
    fn compute_innovation(from: usize, to: usize) -> usize {
        let x = (from + to) * (from + to + 1);
        (x / 2) + to
    }

    pub fn from(&self) -> usize {
        self.from
    }

    pub fn to(&self) -> usize {
        self.to
    }

    pub fn weight(&self) -> Weight {
        self.weight
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn innovation(&self) -> usize {
        self.innovation
    }

    pub fn set_weight(&mut self, weight: Weight) {
        self.weight = weight;
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn activate(&mut self) {
        self.set_active(true)
    }

    pub fn deactivate(&mut self) {
        self.set_active(false)
    }
}

impl PartialEq for Synapse {
    fn eq(&self, other: &Self) -> bool {
        self.innovation == other.innovation
            && self.active == other.active
            && (self.weight - other.weight).abs() < Weight::new(f64::EPSILON).unwrap()
    }
}

impl Eq for Synapse {}

impl Hash for Synapse {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.innovation.hash(state);
    }
}

pub type Synapses = [Synapse];

pub trait SynapsesExt {
    fn get_active_indices(&self) -> HashSet<usize>;
    fn get_active_from_to(&self) -> Vec<(usize, usize)>;
}

impl SynapsesExt for Synapses {
    fn get_active_indices(&self) -> HashSet<usize> {
        HashSet::from_iter(
            self.iter()
                .enumerate()
                .filter_map(|(i, synapse)| (synapse.active()).then(|| i)),
        )
    }

    fn get_active_from_to(&self) -> Vec<(usize, usize)> {
        self.iter()
            .filter(|syn| syn.active())
            .map(|syn| (syn.from(), syn.to()))
            .collect()
    }
}

pub fn create_synapses(links: &[(usize, usize)]) -> Result<Vec<Synapse>, BrainError> {
    links
        .iter()
        .map(|(from, to)| Synapse::new(*from, *to))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::Synapse;
    use crate::weight::Weight;

    #[test]
    fn not_equal_by_innovation() {
        let weight = Weight::new(0_f64).unwrap();
        let a = Synapse::with_weight(0, 1, weight).unwrap();
        let b = Synapse::with_weight(0, 2, weight).unwrap();

        assert_ne!(a, b)
    }

    #[test]
    fn not_equal_by_weight() {
        let weight = Weight::new(0_f64).unwrap();
        let second_weight = Weight::new(0.5).unwrap();
        let a = Synapse::with_weight(0, 1, weight).unwrap();
        let b = Synapse::with_weight(0, 1, second_weight).unwrap();

        assert_ne!(a, b)
    }

    #[test]
    fn not_equal_by_activity() {
        let w = Weight::new(0.5).unwrap();
        let a = Synapse::with_weight(0, 1, w).unwrap();
        let mut b = Synapse::with_weight(0, 1, w).unwrap();

        b.set_active(false);

        assert_ne!(a, b)
    }

    #[test]
    #[should_panic(expected = "value: InvalidFromTo")]
    fn synapse_same_from_to() {
        Synapse::new(0, 0).unwrap();
    }
}
