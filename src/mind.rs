use std::ops::{Deref, DerefMut};

use bevy::prelude::*;
use genesis_brain::Brain;

use crate::{
    body::{EnergyStore, Health},
    config,
};

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
    pub fn new(mind: Mind) -> Self {
        let input_vec = MindInput(vec![0.0; mind.inputs()]);
        let output_vec = MindOutput(vec![0.0; mind.outputs()]);

        Self {
            input: input_vec,
            mind,
            output: output_vec,
        }
    }
    pub fn random(input: usize, output: usize) -> Self {
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

const CONST: f64 = 1.0;

pub fn sensory_system(mut query: Query<(&mut MindInput, &MindOutput, &EnergyStore, &Health)>) {
    for (mut input, output, energy, health) in query.iter_mut() {
        input[config::CONSTANT_INDEX] = CONST;
        input[config::PREV_MOVEMENT_INDEX] = output[config::MOVEMENT_INDEX];
        input[config::PREV_ROTATE_INDEX] = output[config::ROTATE_INDEX];
        input[config::ENERGY_INDEX] = energy.reserve.proportion();
        input[config::HEALTH_INDEX] = health.reserve.proportion();
    }
}

pub fn thinking_system(mut query: Query<(&MindInput, &Mind, &mut MindOutput)>) {
    for (input, bug_brain, mut output) in query.iter_mut() {
        let x = bug_brain.activate(input).expect("Wrong length vector");
        output.0 = x;
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use super::thinking_system;
    use crate::mind::{Mind, MindBundle, MindInput, MindOutput};

    #[test]
    fn mind_thinks() {
        let mut app = App::new();

        app.add_system(thinking_system);

        let mut test_brain = genesis_brain::Brain::new(1, 1);

        test_brain.add_random_synapse();

        let bug_id = app
            .world
            .spawn()
            .insert(Mind(test_brain))
            .insert(MindInput(vec![1.0]))
            .insert(MindOutput(vec![0.0]))
            .id();

        app.update();

        let result = app.world.get::<MindOutput>(bug_id).unwrap();

        assert_ne!(result.0[0], 0.0);
    }

    #[test]
    fn mind_bundle_works() {
        let mut app = App::new();

        app.add_system(thinking_system);

        let bug_id = app
            .world
            .spawn()
            .insert_bundle(MindBundle::random(3, 2))
            .id();

        app.update();

        let mind_in = app.world.get::<MindInput>(bug_id).unwrap();
        let mind_out = app.world.get::<MindOutput>(bug_id).unwrap();

        assert_eq!(mind_in.0.len(), 3);
        assert_eq!(mind_out.0.len(), 2);
    }
}
