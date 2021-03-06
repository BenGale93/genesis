mod activation;
pub mod brain_error;
mod graph;
pub mod neuron;
pub mod synapse;

use activation::ActivationFunctionKind;
pub use brain_error::BrainError;
use genesis_util::{Bias, Weight};
use graph::feed_forward_layers;
pub use neuron::{Neuron, NeuronKind, Neurons, NeuronsExt};
use rand::{prelude::*, seq::SliceRandom};
use rand_distr::StandardNormal;
use synapse::SynapsesExt;
pub use synapse::{create_synapses, Synapse, Synapses};

#[derive(Debug, PartialEq, Eq)]
pub struct Brain {
    inputs: usize,
    outputs: usize,
    neurons: Vec<Neuron>,
    synapses: Vec<Synapse>,
}

impl Brain {
    pub fn new(inputs: usize, outputs: usize) -> Self {
        let mut neurons = vec![];

        (0..inputs).for_each(|_| neurons.push(Neuron::new(NeuronKind::Input)));
        (0..outputs).for_each(|_| neurons.push(Neuron::new(NeuronKind::Output)));

        Self {
            inputs,
            outputs,
            neurons,
            synapses: vec![],
        }
    }

    pub fn inputs(&self) -> usize {
        self.inputs
    }

    pub fn outputs(&self) -> usize {
        self.outputs
    }

    pub fn neurons(&self) -> &[Neuron] {
        self.neurons.as_ref()
    }

    pub fn synapses(&self) -> &[Synapse] {
        self.synapses.as_ref()
    }

    pub fn activate(&self, input_values: &[f64]) -> Result<Vec<f64>, BrainError> {
        if input_values.len() != self.inputs {
            return Err(BrainError::InputArrayError);
        }
        let mut stored_values = vec![0.0; self.neurons.len()];
        for (i, val) in input_values.iter().enumerate() {
            stored_values[i] = *val;
        }

        let layers = feed_forward_layers(self.neurons(), self.synapses());

        for layer in layers {
            for neuron_index in layer {
                let neuron = &self.neurons[neuron_index];
                let incoming_values: Vec<f64> = self
                    .synapses
                    .iter()
                    .filter(|syn| syn.to() == neuron_index)
                    .map(|syn| {
                        let incoming_value = stored_values[syn.from()];
                        incoming_value * syn.weight().as_float()
                    })
                    .collect();
                let final_value: f64 =
                    incoming_values.iter().sum::<f64>() + neuron.bias().as_float();
                stored_values[neuron_index] = neuron.activate(final_value);
            }
        }

        Ok(stored_values[self.inputs..(self.inputs + self.outputs)].to_vec())
    }

    pub fn add_random_synapse(&mut self) {
        let existing_from_to = self.synapses.get_active_from_to();

        let mut possible_from_to: Vec<(usize, usize)> = (0..self.neurons.len())
            .flat_map(|i| {
                let mut inner = vec![];

                (self.inputs..self.neurons.len()).for_each(|j| {
                    if i != j {
                        if !existing_from_to.contains(&(i, j)) {
                            inner.push((i, j));
                        };
                        if !existing_from_to.contains(&(j, i)) {
                            inner.push((j, i));
                        };
                    }
                });

                inner
            })
            .collect();

        possible_from_to.sort_unstable();
        possible_from_to.dedup();

        possible_from_to.retain(|(i, j)| self.can_connect(*i, *j));

        let picked_from_to = possible_from_to.choose(&mut rand::thread_rng());
        if let Some(from_to) = picked_from_to {
            self.add_synapse(from_to.0, from_to.1, Weight::random())
                .unwrap();
        }
    }

    pub fn deactivate_random_synapse(&mut self) {
        let eligible_indexes: Vec<usize> = self
            .synapses()
            .iter()
            .enumerate()
            .filter(|(_, syn)| {
                if !syn.active() {
                    return false;
                }
                let from_index = syn.from();
                let to_index = syn.to();

                let num_outgoing_synapses = self.synapses.num_outgoing_synapses(from_index);
                let num_incoming_synapses = self.synapses.num_incoming_synapses(to_index);

                num_outgoing_synapses > 1 && num_incoming_synapses > 1
            })
            .map(|(i, _)| i)
            .collect();

        let index = eligible_indexes.choose(&mut rand::thread_rng());
        if let Some(i) = index {
            self.deactivate_synapse(*i).unwrap();
        }
    }

    pub fn add_random_neuron(&mut self) {
        let active_synapse_indices = self.synapses.get_active_indices();
        let index = active_synapse_indices.choose(&mut rand::thread_rng());
        if let Some(i) = index {
            self.add_neuron(*i).unwrap();
        }
    }

    pub fn deactivate_random_neuron(&mut self) {
        let hidden_neurons: Vec<usize> = self
            .neurons
            .iter()
            .enumerate()
            .filter(|(i, neuron)| {
                if !matches!(neuron.kind(), NeuronKind::Hidden) {
                    return false;
                }
                let outgoing_count = self.synapses.num_outgoing_synapses(*i);
                let incoming_count = self.synapses.num_incoming_synapses(*i);

                incoming_count > 0 && outgoing_count > 0
            })
            .map(|(i, _)| i)
            .collect();

        let index = hidden_neurons.choose(&mut rand::thread_rng());
        if let Some(i) = index {
            self.remove_neuron(*i).unwrap();
        }
    }

    pub fn mutate_synapse_weight(&mut self) {
        let random_synapse = self.synapses.choose_mut(&mut rand::thread_rng());
        if let Some(syn) = random_synapse {
            let offset: f64 = thread_rng().sample(StandardNormal);
            let new_weight =
                Weight::new((syn.weight().as_float() + offset).min(1.0).max(-1.0)).unwrap();
            syn.set_weight(new_weight);
        }
    }

    pub fn mutate_neuron_bias(&mut self) {
        let mut non_input_neurons: Vec<&mut Neuron> = self
            .neurons
            .iter_mut()
            .filter(|n| !matches!(n.kind(), NeuronKind::Input))
            .collect();

        let random_neuron = non_input_neurons
            .choose_mut(&mut rand::thread_rng())
            .unwrap();

        let offset: f64 = thread_rng().sample(StandardNormal);
        let new_bias = Bias::new(
            (random_neuron.bias().as_float() + offset)
                .min(1.0)
                .max(-1.0),
        )
        .unwrap();

        random_neuron.set_bias(new_bias);
    }

    pub fn mutate_neuron_activation(&mut self) {
        let mut non_input_neurons: Vec<&mut Neuron> = self
            .neurons
            .iter_mut()
            .filter(|n| !matches!(n.kind(), NeuronKind::Input))
            .collect();

        let random_neuron = non_input_neurons
            .choose_mut(&mut rand::thread_rng())
            .unwrap();

        random_neuron.set_activation(random::<ActivationFunctionKind>());
    }

    fn can_connect(&self, from: usize, to: usize) -> bool {
        let from_kind = match self.neurons.get(from) {
            Some(n) => n.kind(),
            None => return false,
        };
        let to_kind = match self.neurons.get(to) {
            Some(n) => n.kind(),
            None => return false,
        };

        if matches!(from_kind, NeuronKind::Output)
            || matches!(to_kind, NeuronKind::Input)
            || (from_kind == to_kind && to_kind != &NeuronKind::Hidden)
            || graph::creates_cycle(&self.synapses, from, to)
        {
            return false;
        }
        true
    }

    fn add_synapse(&mut self, from: usize, to: usize, weight: Weight) -> Result<usize, BrainError> {
        let new_synapse = Synapse::with_weight(from, to, weight)?;

        if self.synapses.contains(&new_synapse) {
            return Err(BrainError::SynapseError);
        }

        if !self.can_connect(from, to) {
            return Err(BrainError::SynapseError);
        }

        let maybe_connection = self
            .synapses
            .iter_mut()
            .find(|syn| syn.innovation() == new_synapse.innovation());

        if let Some(conn) = maybe_connection {
            conn.activate();
        } else {
            self.synapses.push(new_synapse);
        }
        Ok(self.synapses.len() - 1)
    }

    fn deactivate_synapse(&mut self, synapse_index: usize) -> Result<(), BrainError> {
        if self.synapses.get(synapse_index).is_none() {
            return Err(BrainError::OutOfBounds);
        }
        self.deactivate_synapse_unchecked(synapse_index);

        let start_neuron_index: usize;
        let end_neuron_index: usize;

        {
            let deactive_synapse = self.synapses.get(synapse_index).unwrap();
            start_neuron_index = deactive_synapse.from();
            end_neuron_index = deactive_synapse.to();
        }

        {
            let start_neuron = self.neurons.get(start_neuron_index).unwrap();
            if matches!(start_neuron.kind(), NeuronKind::Hidden) {
                let num_outgoing_synapses: usize =
                    self.synapses.num_outgoing_synapses(start_neuron_index);
                if num_outgoing_synapses == 0 {
                    self.remove_neuron(start_neuron_index)?;
                }
            }
        }
        {
            let end_neuron = self.neurons.get(end_neuron_index).unwrap();
            if matches!(end_neuron.kind(), NeuronKind::Hidden) {
                let num_incoming_synapses: usize =
                    self.synapses.num_incoming_synapses(end_neuron_index);
                if num_incoming_synapses == 0 {
                    self.remove_neuron(end_neuron_index)?;
                }
            }
        }

        Ok(())
    }

    fn add_synapse_unchecked(&mut self, from: usize, to: usize, weight: Weight) {
        let new_synapse = Synapse::with_weight(from, to, weight).unwrap();
        self.synapses.push(new_synapse);
    }

    fn deactivate_synapse_unchecked(&mut self, synapse_index: usize) {
        self.synapses.get_mut(synapse_index).unwrap().deactivate();
    }

    fn add_neuron(&mut self, synapse_index: usize) -> Result<usize, BrainError> {
        let target_from: usize;
        let target_to: usize;
        let target_weight: Weight;
        {
            let target_synapse = self
                .synapses
                .get_mut(synapse_index)
                .ok_or(BrainError::OutOfBounds)?;

            if !target_synapse.active() {
                return Err(BrainError::NeuronError);
            }

            target_synapse.deactivate();
            target_from = target_synapse.from();
            target_to = target_synapse.to();
            target_weight = target_synapse.weight();
        }

        let new_neuron_index = self.neurons.len();

        self.neurons.push(Neuron::new(NeuronKind::Hidden));

        self.add_synapse_unchecked(target_from, new_neuron_index, target_weight);
        self.add_synapse_unchecked(new_neuron_index, target_to, target_weight);
        Ok(new_neuron_index)
    }

    fn remove_neuron(&mut self, neuron_index: usize) -> Result<(), BrainError> {
        {
            let neuron_to_remove = match self.neurons.get(neuron_index) {
                Some(neuron) => neuron,
                None => return Err(BrainError::OutOfBounds),
            };

            if !matches!(neuron_to_remove.kind(), NeuronKind::Hidden) {
                return Err(BrainError::NeuronRemovalError);
            }
        }
        let incoming_synapses: Vec<(usize, Weight)> = self
            .synapses
            .iter()
            .filter(|syn| syn.to() == neuron_index && syn.active())
            .map(|syn| (syn.from(), syn.weight()))
            .collect();
        let outgoing_synapses: Vec<usize> = self
            .synapses
            .iter()
            .filter(|syn| syn.from() == neuron_index && syn.active())
            .map(|syn| syn.to())
            .collect();

        let new_from_to_pairs: Vec<(usize, usize, Weight)> = incoming_synapses
            .iter()
            .flat_map(|(from, w)| {
                outgoing_synapses
                    .iter()
                    .map(|to| (*from, *to, *w))
                    .collect::<Vec<(usize, usize, Weight)>>()
            })
            .filter(|(from, to, _)| {
                !self
                    .synapses
                    .iter()
                    .any(|syn| syn.from() == *from && syn.to() == *to && syn.active())
            })
            .collect();
        for (from, to, w) in new_from_to_pairs {
            self.add_synapse(from, to, w)?;
        }

        let synapse_indices_to_deactivate: Vec<usize> = self
            .synapses
            .iter()
            .enumerate()
            .filter_map(|(i, syn)| {
                ((syn.to() == neuron_index || syn.from() == neuron_index) && syn.active())
                    .then_some(i)
            })
            .collect();

        for i in synapse_indices_to_deactivate {
            self.deactivate_synapse(i)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use genesis_util::Weight;

    use crate::{activation::ActivationFunctionKind, SynapsesExt};

    #[test]
    fn add_new_synapse_from_out_to_in() {
        let mut test_brain = super::Brain::new(3, 2);
        let w = Weight::new(1.0).unwrap();

        assert_eq!(0, test_brain.add_synapse(1, 3, w).unwrap());
    }

    #[test]
    #[should_panic(expected = "value: SynapseError")]
    fn add_new_synapse_from_in_to_in() {
        let mut test_brain = super::Brain::new(3, 2);
        let w = Weight::new(1.0).unwrap();

        test_brain.add_synapse(1, 2, w).unwrap();
    }

    #[test]
    #[should_panic(expected = "value: SynapseError")]
    fn add_synapse_already_present_and_active() {
        let mut test_brain = super::Brain::new(3, 2);
        let w = Weight::new(1.0).unwrap();

        test_brain.add_synapse(1, 3, w).unwrap();
        test_brain.add_synapse(1, 3, w).unwrap();
    }

    #[test]
    fn add_synapse_already_present_but_inactive() {
        let mut test_brain = super::Brain::new(3, 2);
        let w = Weight::new(1.0).unwrap();

        test_brain.add_synapse(1, 3, w).unwrap();
        test_brain.deactivate_synapse_unchecked(0);
        assert_eq!(test_brain.add_synapse(1, 3, w).unwrap(), 0);
    }

    #[test]
    #[should_panic(expected = "value: SynapseError")]
    fn add_new_synapse_from_an_output() {
        let mut test_brain = super::Brain::new(3, 2);
        let w = Weight::new(1.0).unwrap();

        test_brain.add_synapse(3, 1, w).unwrap();
    }

    #[test]
    fn deactivate_synapse_in_to_out() {
        let mut test_brain = super::Brain::new(1, 1);
        let w = Weight::new(1.0).unwrap();

        test_brain.add_synapse(0, 1, w).unwrap();
        test_brain.deactivate_synapse(0).unwrap();

        assert_eq!(test_brain.synapses().get_active_indices().len(), 0);
    }

    #[test]
    fn deactivate_synapse_isolated_neuron_no_incoming() {
        let mut test_brain = super::Brain::new(1, 1);
        let w = Weight::new(1.0).unwrap();

        test_brain.add_synapse(0, 1, w).unwrap();
        test_brain.add_neuron(0).unwrap();
        test_brain.deactivate_synapse(1).unwrap();

        assert_eq!(test_brain.synapses().get_active_indices().len(), 0);
    }

    #[test]
    fn deactivate_synapse_isolated_neuron_no_outgoing() {
        let mut test_brain = super::Brain::new(1, 1);
        let w = Weight::new(1.0).unwrap();

        test_brain.add_synapse(0, 1, w).unwrap();
        test_brain.add_neuron(0).unwrap();
        test_brain.deactivate_synapse(2).unwrap();

        assert_eq!(test_brain.synapses().get_active_indices().len(), 0);
    }

    #[test]
    fn deactivate_synapse_isolated_neurons() {
        let mut test_brain = super::Brain::new(1, 1);
        let w = Weight::new(1.0).unwrap();

        test_brain.add_synapse(0, 1, w).unwrap();
        test_brain.add_neuron(0).unwrap();
        test_brain.add_neuron(1).unwrap();
        test_brain.deactivate_synapse(3).unwrap();

        assert_eq!(test_brain.synapses().get_active_indices().len(), 0);
    }

    #[test]
    #[should_panic(expected = "value: OutOfBounds")]
    fn deactivate_synapse_non_existent() {
        let mut test_brain = super::Brain::new(1, 1);

        test_brain.deactivate_synapse(0).unwrap();
    }

    #[test]
    fn add_neuron_success() {
        let mut test_brain = super::Brain::new(3, 2);
        let w = Weight::new(1.0).unwrap();

        test_brain.add_synapse(1, 3, w).unwrap();

        assert_eq!(5, test_brain.add_neuron(0).unwrap());
    }

    #[test]
    #[should_panic(expected = "value: OutOfBounds")]
    fn add_neuron_non_existent_synapse() {
        let mut test_brain = super::Brain::new(3, 2);
        let w = Weight::new(1.0).unwrap();

        test_brain.add_synapse(1, 3, w).unwrap();

        test_brain.add_neuron(1).unwrap();
    }

    #[test]
    #[should_panic(expected = "value: NeuronError")]
    fn add_neuron_deactive_synapse() {
        let mut test_brain = super::Brain::new(1, 1);
        let w = Weight::new(1.0).unwrap();

        test_brain.add_synapse(0, 1, w).unwrap();
        test_brain.deactivate_synapse(0).unwrap();

        test_brain.add_neuron(0).unwrap();
    }

    #[test]
    fn remove_neuron_reactivates_original_synapse() {
        let mut test_brain = super::Brain::new(1, 1);
        let w = Weight::new(1.0).unwrap();

        test_brain.add_synapse(0, 1, w).unwrap();
        assert!(test_brain.synapses().first().unwrap().active());

        test_brain.add_neuron(0).unwrap();
        assert!(!test_brain.synapses().first().unwrap().active());

        test_brain.remove_neuron(2).unwrap();
        assert!(test_brain.synapses().first().unwrap().active());
    }

    #[test]
    fn remove_neuron_with_multiple_in_and_out() {
        let mut test_brain = super::Brain::new(3, 3);
        let w = Weight::new(1.0).unwrap();

        test_brain.add_synapse(0, 3, w).unwrap();
        test_brain.add_neuron(0).unwrap();

        test_brain.add_synapse(1, 6, w).unwrap();
        test_brain.add_synapse(2, 6, w).unwrap();

        test_brain.add_synapse(6, 4, w).unwrap();
        test_brain.add_synapse(6, 5, w).unwrap();

        assert_eq!(test_brain.synapses().get_active_indices().len(), 6);

        test_brain.remove_neuron(6).unwrap();

        assert_eq!(test_brain.synapses().get_active_indices().len(), 9);
    }

    #[test]
    fn remove_neuron_after_synapse_re_activated() {
        let mut test_brain = super::Brain::new(1, 1);
        let w = Weight::new(1.0).unwrap();

        test_brain.add_synapse(0, 1, w).unwrap();
        test_brain.add_neuron(0).unwrap();
        test_brain.add_synapse(0, 1, w).unwrap();
        test_brain.remove_neuron(2).unwrap();

        assert_eq!(test_brain.synapses().get_active_indices().len(), 1);
    }

    #[test]
    #[should_panic(expected = "value: OutOfBounds")]
    fn remove_neuron_nonexistent() {
        let mut test_brain = super::Brain::new(1, 1);

        test_brain.remove_neuron(3).unwrap();
    }

    #[test]
    #[should_panic(expected = "value: NeuronRemovalError")]
    fn remove_neuron_not_hidden() {
        let mut test_brain = super::Brain::new(1, 1);

        test_brain.remove_neuron(0).unwrap();
    }

    #[test]
    fn basic_activate() {
        let mut test_brain = super::Brain::new(1, 1);
        let w = Weight::new(1.0).unwrap();

        test_brain.add_synapse(0, 1, w).unwrap();

        let result = test_brain.activate(&[10.0]).unwrap();

        assert_ne!(result, vec![0.0]);
    }

    #[test]
    fn activate_with_hidden_neuron() {
        let mut test_brain = super::Brain::new(1, 1);
        let w = Weight::new(1.0).unwrap();

        test_brain.add_synapse(0, 1, w).unwrap();
        test_brain.add_neuron(0).unwrap();

        let result = test_brain.activate(&[10.0]).unwrap();

        assert_ne!(result, vec![0.0]);
    }

    #[test]
    fn activate_with_unconnected_output() {
        let mut test_brain = super::Brain::new(2, 2);
        let w = Weight::new(1.0).unwrap();

        test_brain.add_synapse(0, 2, w).unwrap();

        let result = test_brain.activate(&[10.0, -10.0]).unwrap();

        assert_ne!(result[0], 0.0);
        assert_eq!(result[1], 0.0);
    }

    #[test]
    #[should_panic(expected = "value: InputArrayError")]
    fn activate_with_wrong_length_input() {
        let test_brain = super::Brain::new(2, 2);
        test_brain.activate(&[10.0]).unwrap();
    }

    #[test]
    fn add_random_synapse_basic() {
        let mut test_brain = super::Brain::new(3, 3);

        test_brain.add_random_synapse();

        assert_eq!(1, test_brain.synapses().len());
    }

    #[test]
    fn add_random_synapse_hidden_present() {
        let mut test_brain = super::Brain::new(3, 3);

        test_brain.add_random_synapse();
        test_brain.add_neuron(0).unwrap();
        test_brain.add_random_synapse();

        assert_eq!(3, test_brain.synapses().get_active_indices().len());
    }

    #[test]
    fn deactivate_random_synapse_no_changes() {
        let mut test_brain = super::Brain::new(3, 3);

        test_brain.add_random_synapse();
        test_brain.add_neuron(0).unwrap();
        test_brain.deactivate_random_synapse();

        assert_eq!(2, test_brain.synapses().get_active_indices().len());
    }

    #[test]
    fn deactivate_random_synapse_changes() {
        let mut test_brain = super::Brain::new(3, 3);
        let w = Weight::new(1.0).unwrap();

        test_brain.add_synapse(0, 3, w).unwrap();
        test_brain.add_neuron(0).unwrap();
        test_brain.add_synapse(1, 6, w).unwrap();
        test_brain.add_synapse(6, 5, w).unwrap();
        test_brain.add_synapse(0, 4, w).unwrap();
        dbg!(&test_brain.synapses());
        test_brain.deactivate_random_synapse();

        assert_eq!(4, test_brain.synapses().get_active_indices().len());
    }

    #[test]
    fn add_random_neuron_no_options() {
        let mut test_brain = super::Brain::new(3, 3);
        test_brain.add_random_neuron();

        assert_eq!(6, test_brain.neurons().len());
    }

    #[test]
    fn add_random_neuron_single_option() {
        let mut test_brain = super::Brain::new(3, 3);
        let w = Weight::new(1.0).unwrap();

        test_brain.add_synapse(0, 3, w).unwrap();
        test_brain.add_random_neuron();

        assert_eq!(7, test_brain.neurons().len());
        assert_eq!(3, test_brain.synapses().len());
    }

    #[test]
    fn add_random_neuron_multiple_options() {
        let mut test_brain = super::Brain::new(3, 3);
        let w = Weight::new(1.0).unwrap();

        test_brain.add_synapse(0, 3, w).unwrap();
        test_brain.add_synapse(1, 4, w).unwrap();
        test_brain.add_random_neuron();

        assert_eq!(7, test_brain.neurons().len());
        assert_eq!(4, test_brain.synapses().len());
    }

    #[test]
    fn deactivate_random_neuron_no_options() {
        let mut test_brain = super::Brain::new(3, 3);
        test_brain.deactivate_random_neuron();

        assert_eq!(6, test_brain.neurons().len());
    }

    #[test]
    fn deactivate_random_neuron_single_option() {
        let mut test_brain = super::Brain::new(3, 3);
        let w = Weight::new(1.0).unwrap();

        test_brain.add_synapse(0, 3, w).unwrap();
        test_brain.add_neuron(0).unwrap();
        test_brain.deactivate_random_neuron();

        assert_eq!(3, test_brain.synapses().len());
        assert_eq!(1, test_brain.synapses().get_active_indices().len());
    }

    #[test]
    fn deactivate_random_neuron_multiple_options() {
        let mut test_brain = super::Brain::new(3, 3);
        let w = Weight::new(1.0).unwrap();

        test_brain.add_synapse(0, 3, w).unwrap();
        test_brain.add_synapse(1, 4, w).unwrap();
        test_brain.add_neuron(0).unwrap();
        test_brain.add_neuron(1).unwrap();
        test_brain.deactivate_random_neuron();

        assert_eq!(6, test_brain.synapses().len());
        assert_eq!(3, test_brain.synapses().get_active_indices().len());
    }

    #[test]
    fn mutate_synapse_weight_no_synapse_does_not_panic() {
        let mut test_brain = super::Brain::new(1, 1);
        test_brain.mutate_synapse_weight();
    }

    #[test]
    fn mutate_synapse_weight_success() {
        let mut test_brain = super::Brain::new(1, 1);
        let w = Weight::new(0.0).unwrap();

        test_brain.add_synapse(0, 1, w).unwrap();

        test_brain.mutate_synapse_weight();

        assert_ne!(0.0, test_brain.synapses()[0].weight().as_float());
    }

    #[test]
    fn mutate_neuron_bias_success() {
        let mut test_brain = super::Brain::new(1, 1);
        let starting_bias = test_brain.neurons()[1].bias();
        test_brain.mutate_neuron_bias();

        assert_ne!(test_brain.neurons()[1].bias(), starting_bias);
    }

    #[test]
    fn mutate_neuron_activation_does_not_change_input() {
        let mut test_brain = super::Brain::new(1, 1);
        test_brain.mutate_neuron_activation();

        assert_eq!(
            test_brain.neurons()[0].activation(),
            &ActivationFunctionKind::Identity
        );
    }
}
