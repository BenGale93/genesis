use bevy::prelude::{Bundle, Color, Component};
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

    pub fn color(&self) -> Color {
        let innovations = self.0.innovations();
        mind_color(innovations)
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

fn mind_color(mut innovations: Vec<usize>) -> Color {
    innovations.sort_unstable();

    let mut rgb: Vec<f32> = vec![0.5, 0.5, 0.5];

    for innovation in innovations.iter() {
        let perturbation = (1.0 / (*innovation as f32).log(10.0)) - 0.12;
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
