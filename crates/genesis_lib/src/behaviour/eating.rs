use bevy::{
    prelude::{Commands, Entity, EventWriter, Mut, Query, Res, Transform, With, Without},
    time::Stopwatch,
};
use bevy_rapier2d::prelude::RapierContext;
use genesis_attributes as attributes;
use genesis_components::{body::Vitality, eat::*, mind::MindOutput, BurntEnergy, Egg, Size};
use genesis_config as config;
use genesis_ecosystem::Food;
use genesis_maths::angle_between;
use genesis_traits::BehaviourTracker;
use iyes_loopless::prelude::FixedTimesteps;

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
    timesteps: Res<FixedTimesteps>,
    mut bug_query: Query<(&mut TryingToEat, &mut EatingSum, &attributes::CostOfEating)>,
) {
    let standard = timesteps.get("standard").unwrap();
    for (mut trying_to_eat, mut eating_sum, cost) in bug_query.iter_mut() {
        trying_to_eat.tick(standard.step);
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
        Mut<Stomach>,
        &Transform,
        &Size,
        Mut<EnergyConsumed>,
        &attributes::MouthWidth,
    ),
    food: &mut (Entity, Mut<Food>, &Transform),
) {
    let (stomach, bug_transform, bug_size, energy_consumed, mouth_width) = bug;
    let (food_entity, food_energy, food_transform) = food;
    let angle_to_food = angle_between(
        &bug_transform.rotation,
        food_transform.translation - bug_transform.translation,
    );
    if angle_to_food.abs() < ***mouth_width {
        let initial_food_energy = food_energy.energy().amount();
        stomach.eat(food_energy, bug_size);
        let consumed = initial_food_energy - food_energy.energy().amount();
        energy_consumed.0 += consumed;
        if consumed > 0 {
            ev_eaten.send(EatenEvent(*food_entity));
        }
        if food_energy.energy().amount() == 0 {
            commands.entity(*food_entity).insert(Eaten);
        }
    }
}

pub type EatingBug<'a> = (
    &'a mut Stomach,
    &'a Transform,
    &'a Size,
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
        let (mut bug, other_collider) = match bug_query.get_mut(contact_pair.collider1()) {
            Ok(b) => (b, contact_pair.collider2()),
            Err(_) => match bug_query.get_mut(contact_pair.collider2()) {
                Ok(b) => (b, contact_pair.collider1()),
                Err(_) => continue,
            },
        };

        if let Ok(mut food) = food_query.get_mut(other_collider) {
            eat_food(&mut commands, &mut ev_eaten, &mut bug, &mut food);
        }
    }
}

pub fn digestion_intensity_system(mut bug_query: Query<(&MindOutput, &mut Stomach)>) {
    for (mind_out, mut stomach) in bug_query.iter_mut() {
        stomach.set_intensity(mind_out[config::DIGEST_FOOD_INDEX]);
    }
}

pub fn digest_food_system(
    mut bug_query: Query<(
        &attributes::FoodPreference,
        &mut Stomach,
        &mut Vitality,
        &mut BurntEnergy,
        &mut EnergyDigested,
        &mut DigestionCost,
    )>,
) {
    for (
        food_preference,
        mut stomach,
        mut vitality,
        mut burnt_energy,
        mut energy_used,
        mut energy_wasted,
    ) in bug_query.iter_mut()
    {
        let (usable_energy, mut waste_energy) = stomach.digest(food_preference, &mut vitality);
        let usable_energy_amount = usable_energy.amount();
        let more_waste_energy = vitality.add_energy(usable_energy);
        energy_used.0 = usable_energy_amount - more_waste_energy.amount();
        waste_energy.add_energy(more_waste_energy);
        energy_wasted.0 = stomach.digestion_cost();
        burnt_energy.add_energy(waste_energy);
    }
}
