use std::collections::HashSet;

use genesis_util::Bias;
use rand::random;

use crate::activation::{self, ActivationFunctionKind};

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum NeuronKind {
    Input,
    Output,
    Hidden,
}

#[derive(Debug)]
pub struct Neuron {
    kind: NeuronKind,
    activation: activation::ActivationFunctionKind,
    bias: Bias,
}

impl Neuron {
    pub fn new(kind: NeuronKind) -> Self {
        let activation = match kind {
            NeuronKind::Input => activation::ActivationFunctionKind::Identity,
            _ => random::<ActivationFunctionKind>(),
        };

        let bias = match kind {
            NeuronKind::Input => Bias::new(0.).unwrap(),
            _ => Bias::random(),
        };

        Self {
            kind,
            activation,
            bias,
        }
    }

    pub fn kind(&self) -> &NeuronKind {
        &self.kind
    }

    pub fn activation(&self) -> &activation::ActivationFunctionKind {
        &self.activation
    }

    pub fn set_activation(&mut self, activation: activation::ActivationFunctionKind) {
        self.activation = activation;
    }

    pub fn bias(&self) -> Bias {
        self.bias
    }

    pub fn set_bias(&mut self, bias: Bias) {
        self.bias = bias;
    }

    pub fn activate(&self, input: f64) -> f64 {
        activation::activate(input, self.activation()) + self.bias().as_float()
    }
}

impl PartialEq for Neuron {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
            && self.activation == other.activation
            && (self.bias - other.bias).abs() < Bias::new(f64::EPSILON).unwrap()
    }
}

impl Eq for Neuron {}

pub type Neurons = [Neuron];

pub trait NeuronsExt {
    fn get_indices(&self, kinds: &HashSet<NeuronKind>) -> HashSet<usize>;
}

impl NeuronsExt for Neurons {
    fn get_indices(&self, kinds: &HashSet<NeuronKind>) -> HashSet<usize> {
        HashSet::from_iter(
            self.iter()
                .enumerate()
                .filter_map(|(i, neuron)| kinds.contains(neuron.kind()).then_some(i)),
        )
    }
}

#[cfg(test)]
mod tests {
    use genesis_util::Bias;

    use super::{Neuron, NeuronKind};
    use crate::activation;

    #[test]
    fn test_activate_input() {
        let neuron = Neuron::new(NeuronKind::Input);
        let input = 1.0;
        assert_eq!(input, neuron.activate(input));
    }

    #[test]
    fn test_activate_sigmoid() {
        let mut neuron = Neuron::new(NeuronKind::Hidden);
        neuron.set_activation(activation::ActivationFunctionKind::Sigmoid);
        neuron.set_bias(Bias::new(0.0).unwrap());
        let input = 1.0;
        assert_ne!(input, neuron.activate(input));
    }
}
