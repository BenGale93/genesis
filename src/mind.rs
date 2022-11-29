use bevy::prelude::{Bundle, Color, Component};
use derive_more::{Deref, DerefMut, From};
use genesis_brain::Brain;
use rand::{rngs::StdRng, SeedableRng};
use rand_distr::{Distribution, Uniform};

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
        let mut first_innovation = match innovations.first() {
            Some(i) => *i,
            None => return Color::WHITE,
        };
        let mut rng = StdRng::seed_from_u64(first_innovation as u64);
        let uniform = Uniform::new(0.0, 1.0);

        let mut r = uniform.sample(&mut rng);
        let mut g = uniform.sample(&mut rng);
        let mut b = uniform.sample(&mut rng);

        for (i, innovation) in self.innovations().iter().skip(1).enumerate() {
            let diff = *innovation as f32 - first_innovation as f32;
            let perturbation = 1.0 / diff;
            let index_mod = i % 3;
            if index_mod == 0 {
                r = (r + perturbation).clamp(0.0, 1.0);
            } else if index_mod == 1 {
                g = (g + perturbation).clamp(0.0, 1.0);
            } else {
                b = (b + perturbation).clamp(0.0, 1.0);
            }
            first_innovation = *innovation;
        }

        Color::rgb(r, g, b)
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
