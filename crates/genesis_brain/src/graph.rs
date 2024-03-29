use std::collections::HashSet;

use cached::proc_macro::cached;

use crate::{Neuron, NeuronKind, NeuronsExt, Synapse, Synapses};

pub fn creates_cycle(synapses: &Synapses, from: usize, to: usize) -> bool {
    let mut visited = HashSet::from([to]);

    loop {
        let mut num_added = 0;
        for synapse in synapses {
            if visited.contains(&synapse.from()) && !visited.contains(&synapse.to()) {
                if synapse.to() == from {
                    return true;
                }

                visited.insert(synapse.to());
                num_added += 1;
            }
        }

        if num_added == 0 {
            return false;
        }
    }
}

#[cached]
pub fn feed_forward_layers(neurons: Vec<Neuron>, synapses: Vec<Synapse>) -> Vec<HashSet<usize>> {
    let required = neurons.get_indices(&HashSet::from([NeuronKind::Hidden, NeuronKind::Output]));
    let mut visited = neurons.get_indices(&HashSet::from([NeuronKind::Input]));

    let mut layers = Vec::new();

    let active_synapses: Vec<&Synapse> =
        synapses.iter().filter(|synapse| synapse.active()).collect();

    loop {
        let candidates = active_synapses.iter().filter_map(|synapse| {
            (visited.contains(&synapse.from()) && !visited.contains(&synapse.to()))
                .then(|| synapse.to())
        });
        let c = candidates.collect::<HashSet<_>>();
        let mut t = HashSet::new();

        for n in c {
            if required.contains(&n)
                && active_synapses
                    .iter()
                    .filter_map(|synapse| (synapse.to() == n).then(|| synapse.from()))
                    .all(|from| visited.contains(&from))
            {
                t.insert(n);
            }
        }

        if t.is_empty() {
            break;
        }

        visited = visited.union(&t).copied().collect();
        layers.push(t);
    }
    layers
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate as brain;

    #[test]
    fn creates_a_cycle() {
        let synapses = brain::create_synapses(&[(0, 1), (1, 2), (2, 3)]).unwrap();

        assert!(super::creates_cycle(&synapses, 1, 0));
    }

    #[test]
    fn does_not_create_a_cycle() {
        let synapses = brain::create_synapses(&[(0, 1), (1, 2), (2, 3)]).unwrap();

        assert!(!super::creates_cycle(&synapses, 0, 1));
    }

    #[test]
    fn feed_forward_single_layer() {
        let neurons = [
            brain::Neuron::new(brain::NeuronKind::Input),
            brain::Neuron::new(brain::NeuronKind::Input),
            brain::Neuron::new(brain::NeuronKind::Output),
        ];

        let synapses = brain::create_synapses(&[(0, 2), (1, 2)]).unwrap();

        let layers = super::feed_forward_layers(neurons.to_vec(), synapses);

        assert_eq!(layers, vec![HashSet::from([2])]);
    }

    #[test]
    fn feed_forward_two_layers() {
        let neurons = [
            brain::Neuron::new(brain::NeuronKind::Input),
            brain::Neuron::new(brain::NeuronKind::Input),
            brain::Neuron::new(brain::NeuronKind::Output),
            brain::Neuron::new(brain::NeuronKind::Hidden),
        ];

        let synapses = brain::create_synapses(&[(0, 3), (1, 3), (3, 2)]).unwrap();

        let layers = super::feed_forward_layers(neurons.to_vec(), synapses);

        assert_eq!(layers, vec![HashSet::from([3]), HashSet::from([2])]);
    }

    #[test]
    fn feed_forward_multi_members() {
        let neurons = [
            brain::Neuron::new(brain::NeuronKind::Input),
            brain::Neuron::new(brain::NeuronKind::Input),
            brain::Neuron::new(brain::NeuronKind::Output),
            brain::Neuron::new(brain::NeuronKind::Output),
            brain::Neuron::new(brain::NeuronKind::Hidden),
            brain::Neuron::new(brain::NeuronKind::Hidden),
        ];

        let synapses = brain::create_synapses(&[(0, 4), (1, 5), (4, 2), (5, 3)]).unwrap();

        let layers = super::feed_forward_layers(neurons.to_vec(), synapses);

        assert_eq!(layers, vec![HashSet::from([4, 5]), HashSet::from([2, 3])]);
    }

    #[test]
    fn feed_forward_unconnected() {
        let neurons = [
            brain::Neuron::new(brain::NeuronKind::Input),
            brain::Neuron::new(brain::NeuronKind::Input),
            brain::Neuron::new(brain::NeuronKind::Output),
            brain::Neuron::new(brain::NeuronKind::Input),
            brain::Neuron::new(brain::NeuronKind::Output),
        ];

        let synapses = brain::create_synapses(&[(0, 2), (1, 2)]).unwrap();

        let layers = super::feed_forward_layers(neurons.to_vec(), synapses);

        assert_eq!(layers, vec![HashSet::from([2])]);
    }

    #[test]
    fn feed_forward_complex() {
        let neurons = [
            brain::Neuron::new(brain::NeuronKind::Input),
            brain::Neuron::new(brain::NeuronKind::Input),
            brain::Neuron::new(brain::NeuronKind::Input),
            brain::Neuron::new(brain::NeuronKind::Input),
            brain::Neuron::new(brain::NeuronKind::Hidden),
            brain::Neuron::new(brain::NeuronKind::Hidden),
            brain::Neuron::new(brain::NeuronKind::Hidden),
            brain::Neuron::new(brain::NeuronKind::Hidden),
            brain::Neuron::new(brain::NeuronKind::Hidden),
            brain::Neuron::new(brain::NeuronKind::Hidden),
            brain::Neuron::new(brain::NeuronKind::Hidden),
            brain::Neuron::new(brain::NeuronKind::Output),
            brain::Neuron::new(brain::NeuronKind::Output),
            brain::Neuron::new(brain::NeuronKind::Output),
            brain::Neuron::new(brain::NeuronKind::Output),
        ];

        let synapses = brain::create_synapses(&[
            (0, 4),
            (1, 4),
            (1, 5),
            (2, 5),
            (2, 6),
            (3, 6),
            (3, 7),
            (4, 8),
            (5, 8),
            (5, 9),
            (5, 10),
            (6, 10),
            (6, 7),
            (8, 11),
            (8, 12),
            (8, 9),
            (9, 10),
            (7, 10),
            (10, 12),
            (10, 13),
        ])
        .unwrap();

        let layers = super::feed_forward_layers(neurons.to_vec(), synapses);

        assert_eq!(
            layers,
            vec![
                HashSet::from([4, 5, 6]),
                HashSet::from([7, 8]),
                HashSet::from([9, 11]),
                HashSet::from([10]),
                HashSet::from([12, 13])
            ]
        );
    }
}
