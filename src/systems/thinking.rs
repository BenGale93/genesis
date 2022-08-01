use bevy::{core::FixedTimestep, prelude::*};

use crate::{components, config};

pub fn thinking_system(
    mut query: Query<(
        &components::MindInput,
        &components::Mind,
        &mut components::MindOutput,
    )>,
) {
    for (input, bug_brain, mut output) in query.iter_mut() {
        let x = bug_brain.activate(input).expect("Wrong length vector");
        output.0 = x;
    }
}

pub fn thinking_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(config::TIME_STEP as f64))
        .with_system(thinking_system)
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use super::thinking_system;
    use crate::components::{Mind, MindBundle, MindInput, MindOutput};

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

        let bug_id = app.world.spawn().insert_bundle(MindBundle::new(3, 2)).id();

        app.update();

        let mind_in = app.world.get::<MindInput>(bug_id).unwrap();
        let mind_out = app.world.get::<MindOutput>(bug_id).unwrap();

        assert_eq!(mind_in.0.len(), 3);
        assert_eq!(mind_out.0.len(), 2);
    }
}
