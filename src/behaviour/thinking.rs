use bevy::prelude::Query;

use super::{sight::Vision, timers};
use crate::{body, config, mind};

const CONST: f64 = 1.0;

pub fn sensory_system(
    mut query: Query<(
        &mut mind::MindInput,
        &mind::MindOutput,
        &body::Vitality,
        &timers::Age,
        &Vision,
        &timers::Heart,
        &timers::InternalTimer,
    )>,
) {
    for (mut input, output, vitality, age, vision, heart, internal_timer) in query.iter_mut() {
        input[config::CONSTANT_INDEX] = CONST;
        input[config::PREV_MOVEMENT_INDEX] = output[config::MOVEMENT_INDEX];
        input[config::PREV_ROTATE_INDEX] = output[config::ROTATE_INDEX];
        input[config::ENERGY_INDEX] = vitality.energy_store().proportion();
        input[config::HEALTH_INDEX] = vitality.health().proportion();
        input[config::AGE_INDEX] = age.elapsed_secs() as f64;
        input[config::VISIBLE_BUGS_INDEX] = vision.visible_bugs() as f64;
        input[config::BUG_ANGLE_SCORE_INDEX] = vision.bug_angle_score() as f64;
        input[config::BUG_DIST_SCORE_INDEX] = vision.bug_dist_score() as f64;
        input[config::VISIBLE_FOOD_INDEX] = vision.visible_food() as f64;
        input[config::FOOD_ANGLE_SCORE_INDEX] = vision.food_angle_score() as f64;
        input[config::FOOD_DIST_SCORE_INDEX] = vision.food_dist_score() as f64;
        input[config::HEARTBEAT_INDEX] = heart.pulse() as f64;
        input[config::INTERNAL_TIMER_INDEX] = internal_timer.elapsed_secs() as f64;
    }
}

pub fn thinking_system(mut query: Query<(&mind::MindInput, &mind::Mind, &mut mind::MindOutput)>) {
    for (input, bug_brain, mut output) in query.iter_mut() {
        let x = bug_brain.activate(input).expect("Wrong length vector");
        output.0 = x;
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use super::*;
    use crate::{
        config,
        mind::{Mind, MindBundle, MindInput, MindOutput},
    };

    #[test]
    fn mind_thinks() {
        let mut app = App::new();

        app.add_system(thinking_system);

        let mut test_mind: Mind = genesis_brain::Brain::new(1, 1).into();

        test_mind.add_random_synapse();

        let bug_id = app
            .world
            .spawn()
            .insert(test_mind)
            .insert(MindInput(vec![1.0]))
            .insert(MindOutput(vec![0.0]))
            .id();

        app.update();

        let result = app.world.get::<MindOutput>(bug_id).unwrap();

        assert_ne!(result.0[0], 0.0);
    }

    #[test]
    fn mind_bundle_works() {
        config::initialize_config();

        let mut app = App::new();

        app.add_system(thinking_system);
        let mind = Mind::random(3, 2);

        let bug_id = app.world.spawn().insert_bundle(MindBundle::new(mind)).id();

        app.update();

        let mind_in = app.world.get::<MindInput>(bug_id).unwrap();
        let mind_out = app.world.get::<MindOutput>(bug_id).unwrap();

        assert_eq!(mind_in.0.len(), 3);
        assert_eq!(mind_out.0.len(), 2);
    }
}