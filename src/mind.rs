use bevy::prelude::{Bundle, Component};
use derive_more::{Deref, DerefMut, From};
use genesis_brain::Brain;

use crate::config;

#[derive(Component, Debug, PartialEq, Eq, Clone, Deref, DerefMut, From)]
pub struct Mind(pub Brain);

impl Mind {
    pub fn random(input: usize, output: usize) -> Self {
        let mut brain = Brain::new(input, output);

        for _ in 0..config::WorldConfig::global().initial_synapse_count {
            brain.add_random_synapse();
        }

        Self(brain)
    }
}

#[derive(Component, Debug, PartialEq, Clone, Deref, DerefMut, From)]
pub struct MindInput(pub Vec<f64>);

#[derive(Component, Debug, PartialEq, Clone, Deref, DerefMut, From)]
pub struct MindOutput(pub Vec<f64>);

#[derive(Bundle, Debug)]
pub struct MindBundle {
    pub input: MindInput,
    pub mind: Mind,
    pub output: MindOutput,
}

impl MindBundle {
    pub fn new(mind: Mind) -> Self {
        let input_vec = MindInput(vec![0.0; mind.inputs()]);
        let output_vec = MindOutput(vec![0.0; mind.outputs()]);

        Self {
            input: input_vec,
            mind,
            output: output_vec,
        }
    }
}
