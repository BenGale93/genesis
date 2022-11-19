use bevy::{
    prelude::{
        Commands, Component, DespawnRecursiveExt, Entity, Query, Res, Transform, With, Without,
    },
    time::{Stopwatch, Time},
};
use bevy_rapier2d::prelude::RapierContext;
use derive_more::{Deref, DerefMut};
use genesis_util::maths;

use super::metabolism::BurntEnergy;
use crate::{attributes, body::Vitality, config, ecosystem::Plant, mind::MindOutput};

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

#[derive(Component, Debug)]
pub struct EatingSum(f32);

impl EatingSum {
    pub fn new() -> Self {
        Self(0.0)
    }

    pub fn add_eating_time(&mut self, time: f32, cost: f32) {
        self.0 += time * cost
    }

    pub fn uint_portion(&mut self) -> usize {
        let eating_floor = self.0.floor();
        self.0 -= eating_floor;

        eating_floor as usize
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
            trying_to_eat.reset()
        }
    }
}

#[derive(Component, Copy, Clone, Debug, Deref, Ord, PartialEq, Eq, PartialOrd)]
pub struct EnergyConsumed(pub usize);

pub fn eating_system(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut bug_query: Query<
        (
            Entity,
            &mut Vitality,
            &Transform,
            &mut BurntEnergy,
            &mut EnergyConsumed,
        ),
        With<TryingToEat>,
    >,
    mut plant_query: Query<(Entity, &mut Plant, &Transform)>,
) {
    for (bug_entity, mut vitality, bug_transform, mut burnt_energy, mut energy_consumed) in
        bug_query.iter_mut()
    {
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
                        let initial_plant_energy = plant_energy.energy().amount();
                        let leftover = vitality.eat(&mut plant_energy);
                        energy_consumed.0 += initial_plant_energy - leftover.amount();
                        burnt_energy.add_energy(leftover);
                        if plant_energy.energy().amount() == 0 {
                            commands.entity(plant_entity).despawn_recursive();
                        }
                    }
                }
            }
        }
    }
}
