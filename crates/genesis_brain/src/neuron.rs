use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
};

use bevy_reflect::{FromReflect, Reflect};
use genesis_newtype::Bias;
use rand::random;
use serde::{Deserialize, Serialize};

use crate::activation::{self, ActivationFunctionKind};

#[derive(
    PartialEq, Eq, Debug, Hash, Clone, Copy, Deserialize, Serialize, Default, Reflect, FromReflect,
)]
pub enum NeuronKind {
    #[default]
    Input,
    Output,
    Hidden,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default, Reflect, FromReflect)]
#[reflect(Hash, PartialEq)]
pub struct Neuron {
    kind: NeuronKind,
    activation: activation::ActivationFunctionKind,
    bias: Bias,
}

impl Neuron {
    #[must_use]
    pub fn new(kind: NeuronKind) -> Self {
        let activation = match kind {
            NeuronKind::Input => activation::ActivationFunctionKind::Identity,
            NeuronKind::Output => activation::ActivationFunctionKind::Tanh,
            NeuronKind::Hidden => random::<ActivationFunctionKind>(),
        };

        let bias = match kind {
            NeuronKind::Input | NeuronKind::Output => Bias::new(0.).unwrap(),
            NeuronKind::Hidden => Bias::random(),
        };

        Self {
            kind,
            activation,
            bias,
        }
    }

    #[must_use]
    pub const fn kind(&self) -> &NeuronKind {
        &self.kind
    }

    #[must_use]
    pub const fn activation(&self) -> &activation::ActivationFunctionKind {
        &self.activation
    }

    #[must_use]
    pub fn activation_mut(&mut self) -> &mut activation::ActivationFunctionKind {
        &mut self.activation
    }

    pub fn set_activation(&mut self, activation: activation::ActivationFunctionKind) {
        self.activation = activation;
    }

    #[must_use]
    pub const fn bias(&self) -> Bias {
        self.bias
    }

    pub fn set_bias(&mut self, bias: Bias) {
        self.bias = bias;
    }

    #[must_use]
    pub fn activate(&mut self, input: f32) -> f32 {
        activation::activate(input, self.activation_mut()) + self.bias().as_float()
    }
}

impl PartialEq for Neuron {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
            && self.activation == other.activation
            && (self.bias - other.bias).abs() < Bias::new(f32::EPSILON).unwrap()
    }
}

impl Eq for Neuron {}

impl Hash for Neuron {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
        self.activation.hash(state);
    }
}

pub type Neurons = [Neuron];

pub trait NeuronsExt {
    fn get_indices(&self, kinds: &HashSet<NeuronKind>) -> HashSet<usize>;
}

impl NeuronsExt for Neurons {
    fn get_indices(&self, kinds: &HashSet<NeuronKind>) -> HashSet<usize> {
        self.iter()
            .enumerate()
            .filter_map(|(i, neuron)| kinds.contains(neuron.kind()).then_some(i))
            .collect::<HashSet<_>>()
    }
}

#[cfg(test)]
mod tests {
    use genesis_newtype::Bias;

    use super::{Neuron, NeuronKind};
    use crate::activation;

    #[test]
    fn test_activate_input() {
        let mut neuron = Neuron::new(NeuronKind::Input);
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
