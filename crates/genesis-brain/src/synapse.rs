use std::hash::{Hash, Hasher};

use genesis_util::Weight;

use crate::BrainError;

#[derive(Debug, Clone, Copy)]
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
        let innovation = Self::compute_innovation(from, to);
        let weight = Weight::random();

        Ok(Self {
            from,
            to,
            weight,
            active: true,
            innovation,
        })
    }

    pub fn with_weight(from: usize, to: usize, weight: Weight) -> Result<Self, BrainError> {
        let mut synapse = Self::new(from, to)?;
        synapse.set_weight(weight);
        Ok(synapse)
    }

    // Cantor Pairing Function
    const fn compute_innovation(from: usize, to: usize) -> usize {
        let x = (from + to) * (from + to + 1);
        (x / 2) + to
    }

    #[must_use]
    pub const fn from(&self) -> usize {
        self.from
    }

    #[must_use]
    pub const fn to(&self) -> usize {
        self.to
    }

    #[must_use]
    pub const fn weight(&self) -> Weight {
        self.weight
    }

    #[must_use]
    pub const fn active(&self) -> bool {
        self.active
    }

    #[must_use]
    pub const fn innovation(&self) -> usize {
        self.innovation
    }

    pub fn set_weight(&mut self, weight: Weight) {
        self.weight = weight;
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn activate(&mut self) {
        self.set_active(true);
    }

    pub fn deactivate(&mut self) {
        self.set_active(false);
    }
}

impl PartialEq for Synapse {
    fn eq(&self, other: &Self) -> bool {
        self.innovation == other.innovation
            && self.active == other.active
            && (self.weight - other.weight).abs() < Weight::new(f32::EPSILON).unwrap()
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
    fn get_active_indices(&self) -> Vec<usize>;
    fn get_active_from_to(&self) -> Vec<(usize, usize)>;
    fn num_outgoing_synapses(&self, from_index: usize) -> usize;
    fn num_incoming_synapses(&self, from_index: usize) -> usize;
}

impl SynapsesExt for Synapses {
    fn get_active_indices(&self) -> Vec<usize> {
        self.iter()
            .enumerate()
            .filter_map(|(i, synapse)| (synapse.active()).then_some(i))
            .collect()
    }

    fn get_active_from_to(&self) -> Vec<(usize, usize)> {
        self.iter()
            .filter(|syn| syn.active())
            .map(|syn| (syn.from(), syn.to()))
            .collect()
    }

    fn num_outgoing_synapses(&self, from_index: usize) -> usize {
        self.iter()
            .filter(|syn| syn.from() == from_index && syn.active())
            .count()
    }

    fn num_incoming_synapses(&self, to_index: usize) -> usize {
        self.iter()
            .filter(|syn| syn.to() == to_index && syn.active())
            .count()
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
    use genesis_util::Weight;

    use super::Synapse;

    #[test]
    fn not_equal_by_innovation() {
        let weight = Weight::new(0_f32).unwrap();
        let a = Synapse::with_weight(0, 1, weight).unwrap();
        let b = Synapse::with_weight(0, 2, weight).unwrap();

        assert_ne!(a, b)
    }

    #[test]
    fn not_equal_by_weight() {
        let weight = Weight::new(0_f32).unwrap();
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
