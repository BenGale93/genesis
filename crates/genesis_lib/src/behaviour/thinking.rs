use bevy::prelude::{Query, Without};
use genesis_components::{body, mind, see::Vision, time, Egg, ThinkingSum};
use genesis_config as config;
use genesis_traits::BehaviourTracker;

const CONST: f32 = 1.0;

pub fn sensory_system(
    mut query: Query<
        (
            &mut mind::MindInput,
            &mind::MindOutput,
            &body::Vitality,
            &time::Age,
            &Vision,
            &time::Heart,
            &time::InternalTimer,
        ),
        Without<Egg>,
    >,
) {
    for (mut input, output, vitality, age, vision, heart, internal_timer) in query.iter_mut() {
        input[config::CONSTANT_INDEX] = CONST;
        input[config::PREV_MOVEMENT_INDEX] = output[config::MOVEMENT_INDEX];
        input[config::PREV_ROTATE_INDEX] = output[config::ROTATE_INDEX];
        input[config::ENERGY_INDEX] = vitality.energy_store().proportion();
        input[config::HEALTH_INDEX] = vitality.health().proportion();
        input[config::AGE_INDEX] = age.elapsed_secs();
        input[config::VISIBLE_BUGS_INDEX] = *vision.visible_bugs() as f32;
        input[config::BUG_ANGLE_SCORE_INDEX] = *vision.bug_angle_score();
        input[config::BUG_DIST_SCORE_INDEX] = *vision.bug_dist_score();
        input[config::VISIBLE_FOOD_INDEX] = *vision.visible_food() as f32;
        input[config::FOOD_ANGLE_SCORE_INDEX] = *vision.food_angle_score();
        input[config::FOOD_DIST_SCORE_INDEX] = *vision.food_dist_score();
        input[config::HEARTBEAT_INDEX] = heart.pulse();
        input[config::INTERNAL_TIMER_INDEX] = internal_timer.elapsed_secs();
    }
}

pub fn thinking_system(
    mut query: Query<(
        &mind::MindInput,
        &mind::Mind,
        &mut mind::MindOutput,
        &mut ThinkingSum,
    )>,
) {
    let cost = config::WorldConfig::global().cost_of_thought;
    for (input, bug_brain, mut output, mut thoughts) in query.iter_mut() {
        let mut result = bug_brain.activate(input).expect("Wrong length vector");
        result[config::MOVEMENT_INDEX] = result[config::MOVEMENT_INDEX].clamp(-1.0, 1.0);
        result[config::ROTATE_INDEX] = result[config::ROTATE_INDEX].clamp(-1.0, 1.0);
        output.0 = result;
        thoughts.add_time(
            config::BEHAVIOUR_TICK.as_secs_f32(),
            bug_brain.synapses().len() as f32 * cost,
        );
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use genesis_components::mind::*;
    use genesis_config as config;
    use genesis_newtype::Weight;

    use super::*;

    #[test]
    fn mind_thinks() {
        config::initialize_configs(None);
        let mut app = App::new();

        app.add_system(thinking_system);

        let mut test_mind: Mind = genesis_brain::Brain::new(10, 10).into();
        let w = Weight::new(1.0).unwrap();

        test_mind.add_synapse(0, 10, w).unwrap();

        let bug_id = app
            .world
            .spawn(test_mind)
            .insert(MindInput(vec![1.0; 10]))
            .insert(MindOutput(vec![0.0; 10]))
            .insert(ThinkingSum::new())
            .id();

        app.update();

        let result = app.world.get::<MindOutput>(bug_id).unwrap();

        assert_ne!(result.0[0], 0.0);
    }

    #[test]
    fn mind_bundle_works() {
        config::initialize_configs(None);

        let mut app = App::new();

        app.add_system(thinking_system);
        let starting_synapses: &[(usize, usize)] = &[];
        let mind = Mind::minimal(3, 2, starting_synapses);

        let bug_id = app.world.spawn(MindBundle::new(&mind)).id();

        app.update();

        let mind_in = app.world.get::<MindInput>(bug_id).unwrap();
        let mind_out = app.world.get::<MindOutput>(bug_id).unwrap();

        assert_eq!(mind_in.0.len(), 3);
        assert_eq!(mind_out.0.len(), 2);
    }
}
