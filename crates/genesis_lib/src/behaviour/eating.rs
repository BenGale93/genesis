use bevy::{
    prelude::{Commands, Entity, EventWriter, Mut, Query, Res, Transform, With, Without},
    time::{Stopwatch, Time},
};
use bevy_rapier2d::prelude::RapierContext;
use genesis_attributes as attributes;
use genesis_components::{body::Vitality, eat::*, mind::MindOutput, BurntEnergy, Egg};
use genesis_config as config;
use genesis_ecosystem::Plant;
use genesis_maths::angle_between;

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

fn eat_plant(
    commands: &mut Commands,
    ev_eaten: &mut EventWriter<EatenEvent>,
    bug: &mut (
        Mut<Vitality>,
        &Transform,
        Mut<BurntEnergy>,
        Mut<EnergyConsumed>,
        &attributes::MouthWidth,
    ),
    plant: &mut (Entity, Mut<Plant>, &Transform),
) {
    let (vitality, bug_transform, burnt_energy, energy_consumed, mouth_width) = bug;
    let (plant_entity, plant_energy, plant_transform) = plant;
    let angle_to_food = angle_between(
        &bug_transform.rotation,
        plant_transform.translation - bug_transform.translation,
    );
    if angle_to_food.abs() < ***mouth_width {
        let initial_plant_energy = plant_energy.energy().amount();
        let leftover = vitality.eat(plant_energy);
        let consumed = initial_plant_energy - plant_energy.energy().amount();
        energy_consumed.0 += consumed;
        if consumed > 0 {
            ev_eaten.send(EatenEvent(*plant_entity));
        }
        if plant_energy.energy().amount() == 0 {
            commands.entity(*plant_entity).insert(Eaten);
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

pub type EatenPlant<'a> = (Entity, &'a mut Plant, &'a Transform);

pub fn eating_system(
    mut commands: Commands,
    mut ev_eaten: EventWriter<EatenEvent>,
    rapier_context: Res<RapierContext>,
    mut bug_query: Query<EatingBug, With<TryingToEat>>,
    mut plant_query: Query<EatenPlant>,
) {
    for contact_pair in rapier_context.contact_pairs() {
        if let (Ok(mut bug), Ok(mut plant)) = (
            bug_query.get_mut(contact_pair.collider1()),
            plant_query.get_mut(contact_pair.collider2()),
        ) {
            eat_plant(&mut commands, &mut ev_eaten, &mut bug, &mut plant);
            continue;
        }
        if let (Ok(mut bug), Ok(mut plant)) = (
            bug_query.get_mut(contact_pair.collider2()),
            plant_query.get_mut(contact_pair.collider1()),
        ) {
            eat_plant(&mut commands, &mut ev_eaten, &mut bug, &mut plant);
        }
    }
}
