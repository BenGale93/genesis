use bevy::{
    prelude::{Commands, Entity, EventWriter, Mut, Query, Res, Transform, With, Without},
    time::{Stopwatch, Time},
};
use bevy_rapier2d::prelude::RapierContext;
use genesis_attributes as attributes;
use genesis_components::{body::Vitality, eat::*, mind::MindOutput, BurntEnergy, Egg};
use genesis_config as config;
use genesis_ecosystem::Food;
use genesis_maths::angle_between;
use genesis_traits::BehaviourTracker;

pub fn process_eaters_system(
    mut commands: Commands,
    not_eating_query: Query<(Entity, &MindOutput), (Without<Egg>, Without<TryingToEat>)>,
    eating_query: Query<(Entity, &MindOutput), With<TryingToEat>>,
) {
    for (entity, mind_out) in not_eating_query.iter() {
        if mind_out[config::EAT_INDEX] >= 0.0 {
            commands
                .entity(entity)
                .insert(TryingToEat(Stopwatch::new()));
        }
    }

    for (entity, mind_out) in eating_query.iter() {
        if mind_out[config::EAT_INDEX] < 0.0 {
            commands.entity(entity).remove::<TryingToEat>();
        }
    }
}

pub fn attempted_to_eat_system(
    time: Res<Time>,
    mut bug_query: Query<(&mut TryingToEat, &mut EatingSum, &attributes::CostOfEating)>,
) {
    for (mut trying_to_eat, mut eating_sum, cost) in bug_query.iter_mut() {
        trying_to_eat.tick(time.delta());
        let time_spent = trying_to_eat.elapsed().as_secs_f32();
        if time_spent >= 1.0 {
            eating_sum.add_time(time_spent, **cost);
            trying_to_eat.reset();
        }
    }
}

fn eat_food(
    commands: &mut Commands,
    ev_eaten: &mut EventWriter<EatenEvent>,
    bug: &mut (
        Mut<Vitality>,
        &Transform,
        Mut<BurntEnergy>,
        Mut<EnergyConsumed>,
        &attributes::MouthWidth,
    ),
    food: &mut (Entity, Mut<Food>, &Transform),
) {
    let (vitality, bug_transform, burnt_energy, energy_consumed, mouth_width) = bug;
    let (food_entity, food_energy, food_transform) = food;
    let angle_to_food = angle_between(
        &bug_transform.rotation,
        food_transform.translation - bug_transform.translation,
    );
    if angle_to_food.abs() < ***mouth_width {
        let initial_food_energy = food_energy.energy().amount();
        let leftover = vitality.eat(food_energy);
        let consumed = initial_food_energy - food_energy.energy().amount();
        energy_consumed.0 += consumed;
        if consumed > 0 {
            ev_eaten.send(EatenEvent(*food_entity));
        }
        if food_energy.energy().amount() == 0 {
            commands.entity(*food_entity).insert(Eaten);
        }
        burnt_energy.add_energy(leftover);
    }
}

pub type EatingBug<'a> = (
    &'a mut Vitality,
    &'a Transform,
    &'a mut BurntEnergy,
    &'a mut EnergyConsumed,
    &'a attributes::MouthWidth,
);

pub type EatenFood<'a> = (Entity, &'a mut Food, &'a Transform);

pub fn eating_system(
    mut commands: Commands,
    mut ev_eaten: EventWriter<EatenEvent>,
    rapier_context: Res<RapierContext>,
    mut bug_query: Query<EatingBug, With<TryingToEat>>,
    mut food_query: Query<EatenFood>,
) {
    for contact_pair in rapier_context.contact_pairs() {
        if let (Ok(mut bug), Ok(mut food)) = (
            bug_query.get_mut(contact_pair.collider1()),
            food_query.get_mut(contact_pair.collider2()),
        ) {
            eat_food(&mut commands, &mut ev_eaten, &mut bug, &mut food);
            continue;
        }
        if let (Ok(mut bug), Ok(mut food)) = (
            bug_query.get_mut(contact_pair.collider2()),
            food_query.get_mut(contact_pair.collider1()),
        ) {
            eat_food(&mut commands, &mut ev_eaten, &mut bug, &mut food);
        }
    }
}
