use bevy::{
    prelude::{Commands, Component, Entity, Query, Res, Transform, With, Without},
    time::Stopwatch,
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
