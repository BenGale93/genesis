use bevy::{prelude::*, time::Stopwatch};
use bevy_rapier2d::prelude::*;
use derive_more::From;
use genesis_brain::Brain;
use genesis_util::maths;

use crate::{
    attributes,
    body::{Age, BugBody, BurntEnergy, Heart, InternalTimer, Vitality},
    config,
    ecosystem::Plant,
    lifecycle,
    sight::Vision,
    spawn,
};
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

const CONST: f64 = 1.0;

pub fn sensory_system(
    mut query: Query<(
        &mut MindInput,
        &MindOutput,
        &Vitality,
        &Age,
        &Vision,
        &Heart,
        &InternalTimer,
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

pub fn thinking_system(mut query: Query<(&MindInput, &Mind, &mut MindOutput)>) {
    for (input, bug_brain, mut output) in query.iter_mut() {
        let x = bug_brain.activate(input).expect("Wrong length vector");
        output.0 = x;
    }
}

pub fn reset_internal_timer_system(
    mut query: Query<(
        &mut InternalTimer,
        &MindOutput,
        &attributes::InternalTimerBoundary,
    )>,
) {
    for (mut internal_timer, mind_out, boundary) in query.iter_mut() {
        if mind_out[config::RESET_TIMER_INDEX] > **boundary {
            internal_timer.reset();
        }
    }
}

pub fn thinking_energy_system(mut query: Query<(&Mind, &mut Vitality, &mut BurntEnergy)>) {
    for (bug_brain, mut vitality, mut burnt_energy) in query.iter_mut() {
        let thought_energy = vitality.take_energy(bug_brain.synapses().len());
        burnt_energy.add_energy(thought_energy);
    }
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct TryingToEat(pub Stopwatch);

pub fn process_eaters_system(
    mut commands: Commands,
    not_eating_query: Query<
        (Entity, &MindOutput, &attributes::EatingBoundary),
        Without<TryingToEat>,
    >,
    eating_query: Query<(Entity, &MindOutput, &attributes::EatingBoundary), With<TryingToEat>>,
) {
    for (entity, mind_out, eating_boundary) in not_eating_query.iter() {
        if mind_out[config::EAT_INDEX] > **eating_boundary {
            commands
                .entity(entity)
                .insert(TryingToEat(Stopwatch::new()));
        }
    }

    for (entity, mind_out, eating_boundary) in eating_query.iter() {
        if mind_out[config::EAT_INDEX] <= **eating_boundary {
            commands.entity(entity).remove::<TryingToEat>();
        }
    }
}

pub fn eating_system(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut bug_query: Query<(Entity, &mut Vitality, &Transform, &mut BurntEnergy), With<TryingToEat>>,
    mut plant_query: Query<(Entity, &mut Plant, &Transform)>,
) {
    for (bug_entity, mut vitality, bug_transform, mut burnt_energy) in bug_query.iter_mut() {
        for contact_pair in rapier_context.contacts_with(bug_entity) {
            let other_collider = if contact_pair.collider1() == bug_entity {
                contact_pair.collider2()
            } else {
                contact_pair.collider1()
            };
            for (plant_entity, mut plant_energy, plant_transform) in plant_query.iter_mut() {
                if other_collider == plant_entity {
                    let angle = maths::angle_to_point(
                        plant_transform.translation - bug_transform.translation,
                    );
                    let rebased_angle = maths::rebased_angle(angle, bug_transform.rotation.z);
                    if rebased_angle < 0.5 {
                        let leftover = vitality.eat(&mut plant_energy);
                        burnt_energy.add_energy(leftover);
                        if plant_energy.energy().amount() == 0 {
                            commands.entity(plant_entity).despawn();
                        }
                    }
                }
            }
        }
    }
}

pub fn attempted_to_eat_system(
    time: Res<Time>,
    mut bug_query: Query<(&mut Vitality, &mut TryingToEat, &mut BurntEnergy)>,
) {
    for (mut vitality, mut trying_to_eat, mut burnt_energy) in bug_query.iter_mut() {
        trying_to_eat.tick(time.delta());
        if trying_to_eat.elapsed().as_secs_f32() >= 1.0 {
            burnt_energy
                .add_energy(vitality.take_energy(config::WorldConfig::global().eating_cost));
            trying_to_eat.reset()
        }
    }
}

#[derive(Component)]
pub struct TryingToLay;

type LayerTest<'a> = (Entity, &'a MindOutput, &'a attributes::LayEggBoundary);

pub fn process_layers_system(
    mut commands: Commands,
    not_laying_query: Query<LayerTest, (Without<TryingToLay>, With<lifecycle::Adult>)>,
    laying_query: Query<LayerTest, (With<TryingToLay>, With<lifecycle::Adult>)>,
) {
    for (entity, mind_out, boundary) in not_laying_query.iter() {
        if mind_out[config::REPRODUCE_INDEX] > **boundary {
            commands.entity(entity).insert(TryingToLay);
        }
    }

    for (entity, mind_out, boundary) in laying_query.iter() {
        if mind_out[config::REPRODUCE_INDEX] <= **boundary {
            commands.entity(entity).remove::<TryingToLay>();
        }
    }
}

fn egg_position(parent_transform: &Transform) -> Vec3 {
    let separation = 20.0;
    let mut egg_pos = parent_transform.translation;
    let angle = parent_transform.rotation.z.asin() * 2.0;
    let (s, c) = angle.sin_cos();

    egg_pos.y -= separation * c;
    egg_pos.x += separation * s;

    egg_pos
}

type Parent<'a> = (
    &'a Transform,
    &'a BugBody,
    &'a Mind,
    &'a attributes::MutationProbability,
    &'a mut Vitality,
    &'a attributes::OffspringEnergy,
    &'a lifecycle::Generation,
);

pub fn lay_egg_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut parent_query: Query<Parent, With<TryingToLay>>,
) {
    let mut rng = rand::thread_rng();
    for (transform, bug_body, mind, prob, mut vitality, offspring_energy, generation) in
        parent_query.iter_mut()
    {
        if vitality.energy_store().amount() < **offspring_energy {
            continue;
        }
        let energy = vitality.take_energy(**offspring_energy);
        let location = egg_position(transform);
        let offspring_body = bug_body.mutate(&mut rng, **prob);
        let offspring_mind = mind.mutate(&mut rng, **prob).into();
        spawn::spawn_egg(
            &mut commands,
            &asset_server,
            energy,
            location,
            offspring_body,
            offspring_mind,
            *generation + 1.into(),
        );
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use super::*;
    use crate::config;

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
