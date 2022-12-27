use bevy_ecs::prelude::{Bundle, Component};
use bevy_egui::egui;
use bevy_render::color::Color;
use derive_more::{Deref, DerefMut, From};
use genesis_brain::{
    feed_forward_layers, ActivationFunctionKind, Brain, NeuronKind, Neurons, Synapses,
};
use genesis_color as color;
use genesis_config as config;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MindValidationError {
    #[error("Not enough neurons")]
    NotEnoughNeurons,
    #[error("Expected input neuron at index '{0}'.")]
    ExpectedInputNeuron(usize),
    #[error("Expected output neuron at index '{0}'.")]
    ExpectedOutputNeuron(usize),
    #[error("Expected hidden neuron at index '{0}'.")]
    ExpectedHiddenNeuron(usize),
    #[error("Invalid input neuron at index '{0}' it should have Identity activation function.")]
    InputNeuronStructure(usize),
    #[error("Invalid output neuron at index '{0}' is should have Tanh activation function.")]
    OutputNeuronStructure(usize),
    #[error("Invalid 'to' index found on synapse '{0}'.")]
    InvalidSynapseTo(usize),
    #[error("Invalid 'from' index found on synapse '{0}'.")]
    InvalidSynapseFrom(usize),
}

#[derive(Component, Debug, PartialEq, Eq, Clone, Deref, DerefMut, From, Deserialize, Serialize)]
pub struct Mind(pub Brain);

impl Mind {
    pub fn random(input: usize, output: usize) -> Self {
        let mut brain = Brain::new(input, output);

        for _ in 0..config::WorldConfig::global().initial_synapse_count {
            brain.add_random_synapse();
        }

        Self(brain)
    }

    pub fn color(&self) -> Color {
        let innovations = self.0.innovations();
        mind_color(innovations)
    }

    pub fn validate(&self) -> Result<(), MindValidationError> {
        let total_non_hidden = config::INPUT_NEURONS + config::OUTPUT_NEURONS;
        if self.neurons().len() < total_non_hidden {
            return Err(MindValidationError::NotEnoughNeurons);
        }
        for (i, neuron) in self.neurons().iter().enumerate() {
            // Check neuron kind.
            if i < config::INPUT_NEURONS && neuron.kind() != &NeuronKind::Input {
                return Err(MindValidationError::ExpectedInputNeuron(i));
            }
            if (config::INPUT_NEURONS..total_non_hidden).contains(&i)
                && neuron.kind() != &NeuronKind::Output
            {
                return Err(MindValidationError::ExpectedOutputNeuron(i));
            }
            if i > total_non_hidden && neuron.kind() != &NeuronKind::Hidden {
                return Err(MindValidationError::ExpectedHiddenNeuron(i));
            }
            // Check activation kind.
            if neuron.kind() == &NeuronKind::Input
                && neuron.activation() != &ActivationFunctionKind::Identity
            {
                return Err(MindValidationError::InputNeuronStructure(i));
            }
            if neuron.kind() == &NeuronKind::Output
                && neuron.activation() != &ActivationFunctionKind::Tanh
            {
                return Err(MindValidationError::OutputNeuronStructure(i));
            }
        }
        for (i, synapse) in self.synapses().iter().enumerate() {
            if !(config::INPUT_NEURONS..self.neurons().len()).contains(&synapse.to()) {
                return Err(MindValidationError::InvalidSynapseTo(i));
            }
            if (config::INPUT_NEURONS..total_non_hidden).contains(&synapse.from())
                || synapse.from() >= self.neurons().len()
            {
                return Err(MindValidationError::InvalidSynapseFrom(i));
            }
        }

        Ok(())
    }
}

#[derive(Component, Debug, PartialEq, Clone, Deref, DerefMut, From)]
pub struct MindInput(pub Vec<f32>);

#[derive(Component, Debug, PartialEq, Clone, Deref, DerefMut, From)]
pub struct MindOutput(pub Vec<f32>);

fn mind_color(mut innovations: Vec<usize>) -> Color {
    innovations.sort_unstable();

    let mut rgb: Vec<f32> = vec![0.5, 0.5, 0.5];

    for innovation in &innovations {
        let perturbation = (1.0 / (*innovation as f32).log10()) - 0.12;
        let index_mod = innovation % 3;
        let sign_mod = innovation % 2;
        if sign_mod == 0 {
            rgb[index_mod] = (rgb[index_mod] + perturbation).clamp(0.0, 1.0);
        } else {
            rgb[index_mod] = (rgb[index_mod] - perturbation).clamp(0.0, 1.0);
        }
    }
    Color::rgb(rgb[0], rgb[1], rgb[2])
}

#[derive(Debug)]
pub struct GuiNeuron {
    pub index: usize,
    pub pos: Option<egui::Pos2>,
    pub color: egui::Color32,
}

#[derive(Debug)]
pub struct PaintedSynapse {
    pub start: egui::Pos2,
    pub end: egui::Pos2,
    pub color: egui::Color32,
}

impl PaintedSynapse {
    pub const fn new(start: egui::Pos2, end: egui::Pos2, color: egui::Color32) -> Self {
        Self { start, end, color }
    }
}

pub const RADIUS: f32 = 20.0;
pub const SPACING: f32 = 20.0;
pub const START_POS: (f32, f32) = (30.0, 100.0);
pub const COLOR_ARRAY: &[(u8, u8, u8)] = &[(255, 0, 0), (160, 160, 160), (0, 150, 0)];

#[derive(Component, Debug)]
pub struct MindLayout {
    neurons: Vec<GuiNeuron>,
    synapses: Vec<PaintedSynapse>,
}

impl MindLayout {
    pub fn new(mind: &Mind) -> Self {
        let neurons =
            Self::layout_neurons(mind.neurons(), mind.synapses(), &START_POS, RADIUS, SPACING);
        let synapses = Self::painted_synapses(mind.synapses(), &neurons);
        Self { neurons, synapses }
    }

    pub fn neurons(&self) -> &[GuiNeuron] {
        self.neurons.as_ref()
    }

    pub fn synapses(&self) -> &[PaintedSynapse] {
        self.synapses.as_ref()
    }

    fn painted_synapses(synapses: &Synapses, neuron_layout: &[GuiNeuron]) -> Vec<PaintedSynapse> {
        let mut painted_synapses = vec![];
        let sorted_synapses = synapses
            .iter()
            .sorted_by(|a, b| a.active().cmp(&b.active()));
        for syn in sorted_synapses {
            let start_pos = &neuron_layout[syn.from()];
            let end_pos = &neuron_layout[syn.to()];

            let (Some(start), Some(end)) = (start_pos.pos, end_pos.pos) else {
            continue;
        };

            let color = if syn.active() {
                let (r, g, b) = color::interpolate_color(syn.weight(), COLOR_ARRAY);
                egui::Color32::from_rgb(r, g, b)
            } else {
                egui::Color32::BLACK
            };
            painted_synapses.push(PaintedSynapse::new(start, end, color));
        }
        painted_synapses
    }

    pub fn layout_neurons(
        neurons: &Neurons,
        synapses: &Synapses,
        start: &(f32, f32),
        radius: f32,
        spacing: f32,
    ) -> Vec<GuiNeuron> {
        let max_layer = 10;
        let impossible_layer = max_layer + 1;
        let layers = feed_forward_layers(neurons.to_vec(), synapses.to_vec());

        let mut positions = Vec::new();
        let total_hidden_layers = layers.len();

        let mut layer_index;
        let mut offsets: Vec<usize> = vec![0; impossible_layer + 1];
        for (k, neuron) in neurons.iter().enumerate() {
            match neuron.kind() {
                NeuronKind::Input => layer_index = 0,
                NeuronKind::Output => layer_index = max_layer,
                NeuronKind::Hidden => {
                    layer_index = impossible_layer;
                    for (i, layer) in layers.iter().enumerate() {
                        if layer.contains(&k) {
                            layer_index = (max_layer / total_hidden_layers) * (i + 1);
                            break;
                        }
                    }
                }
            }

            let offset = &mut offsets[layer_index];

            let pos = if layer_index == impossible_layer {
                None
            } else {
                Some(egui::pos2(
                    (*offset as f32).mul_add(2.0f32.mul_add(radius, spacing), start.0),
                    (layer_index as f32).mul_add(2.0f32.mul_add(radius, spacing), start.1),
                ))
            };
            let (r, g, b) = color::interpolate_color(neuron.bias(), COLOR_ARRAY);
            let color = egui::Color32::from_rgb(r, g, b);
            positions.push(GuiNeuron {
                index: k,
                pos,
                color,
            });

            *offset += 1;
        }
        positions
    }
}

#[derive(Bundle, Debug)]
pub struct MindBundle {
    pub input: MindInput,
    pub output: MindOutput,
    pub layout: MindLayout,
}

impl MindBundle {
    pub fn new(mind: &Mind) -> Self {
        let input_vec = MindInput(vec![0.0; mind.inputs()]);
        let output_vec = MindOutput(vec![0.0; mind.outputs()]);
        let layout = MindLayout::new(mind);

        Self {
            input: input_vec,
            output: output_vec,
            layout,
        }
    }
}

#[cfg(test)]
mod tests {
    use genesis_brain::Brain;
    use genesis_newtype::Weight;

    use super::*;
    #[test]
    fn mind_layout_with_unconnected_neuron() {
        let mut test_brain = Mind(Brain::new(3, 1));
        let w = Weight::new(1.0).unwrap();

        test_brain.add_synapse(0, 3, w).unwrap();
        test_brain.add_neuron(0).unwrap();
        test_brain.add_synapse(1, 3, w).unwrap();
        test_brain.add_synapse(2, 3, w).unwrap();
        test_brain.deactivate_synapse(2).unwrap();

        let layout = MindLayout::layout_neurons(
            test_brain.neurons(),
            test_brain.synapses(),
            &(0.0, 0.0),
            10.0,
            10.0,
        );

        assert_eq!(layout.iter().filter(|x| x.pos.is_some()).count(), 4);
    }
}
