use bevy::{
    prelude::{Commands, Entity, Query, Res, Transform, With, Without},
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
            eating_sum.add_eating_time(time_spent, **cost);
            trying_to_eat.reset();
        }
    }
}

pub type EatingBug<'a> = (
    Entity,
    &'a mut Vitality,
    &'a Transform,
    &'a mut BurntEnergy,
    &'a mut EnergyConsumed,
    &'a attributes::MouthWidth,
);

pub fn eating_system(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut bug_query: Query<EatingBug, With<TryingToEat>>,
    mut plant_query: Query<(Entity, &mut Plant, &Transform)>,
) {
    for (
        bug_entity,
        mut vitality,
        bug_transform,
        mut burnt_energy,
        mut energy_consumed,
        mouth_width,
    ) in bug_query.iter_mut()
    {
        for contact_pair in rapier_context.contacts_with(bug_entity) {
            let other_collider = if contact_pair.collider1() == bug_entity {
                contact_pair.collider2()
            } else {
                contact_pair.collider1()
            };
            for (plant_entity, mut plant_energy, plant_transform) in plant_query.iter_mut() {
                if other_collider == plant_entity {
                    let angle_to_food = angle_between(
                        &bug_transform.rotation,
                        plant_transform.translation - bug_transform.translation,
                    );
                    if angle_to_food.abs() < **mouth_width {
                        let initial_plant_energy = plant_energy.energy().amount();
                        let leftover = vitality.eat(&mut plant_energy);
                        energy_consumed.0 += initial_plant_energy - plant_energy.energy().amount();
                        if plant_energy.energy().amount() == 0 {
                            commands.entity(plant_entity).insert(Eaten);
                        }
                        burnt_energy.add_energy(leftover);
                    }
                }
            }
        }
    }
}
