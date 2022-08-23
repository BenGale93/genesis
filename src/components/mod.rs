use std::ops::{Deref, DerefMut};

use bevy::prelude::{Bundle, Component};
use genesis_brain::Brain;

mod body;

pub use body::BugBody;

use crate::config;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct Mind(pub Brain);

impl Deref for Mind {
    type Target = Brain;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Mind {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Component, Debug, PartialEq, Clone)]
pub struct MindInput(pub Vec<f64>);

impl Deref for MindInput {
    type Target = Vec<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MindInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<f64>> for MindInput {
    fn from(val: Vec<f64>) -> MindInput {
        MindInput(val)
    }
}

#[derive(Component, Debug, PartialEq, Clone)]
pub struct MindOutput(pub Vec<f64>);

impl Deref for MindOutput {
    type Target = Vec<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MindOutput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<f64>> for MindOutput {
    fn from(val: Vec<f64>) -> MindOutput {
        MindOutput(val)
    }
}

#[derive(Bundle, Debug)]
pub struct MindBundle {
    pub input: MindInput,
    pub mind: Mind,
    pub output: MindOutput,
}

impl MindBundle {
    pub fn new(input: usize, output: usize) -> Self {
        let input_vec = MindInput(vec![0.0; input]);
        let output_vec = MindOutput(vec![0.0; output]);
        let mut mind = Mind(Brain::new(input, output));

        for _ in 0..config::INITIAL_SYNAPSE_COUNT {
            mind.add_random_synapse();
        }

        Self {
            input: input_vec,
            mind,
            output: output_vec,
        }
    }
}
