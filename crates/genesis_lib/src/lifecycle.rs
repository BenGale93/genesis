use bevy::prelude::{
    Commands, Component, DespawnRecursiveExt, Entity, Query, ResMut, With, Without,
};
use derive_more::{Add, Deref, DerefMut, From};

use crate::{ancestors, attributes, behaviour::timers, body, ecosystem};

#[derive(Component, Debug)]
pub struct Egg;

#[derive(Component, Debug)]
pub struct Hatching;

#[derive(Component, Debug)]
pub struct Juvenile;

#[derive(Component, Debug)]
pub struct Adult;

#[derive(
    Component, Debug, Deref, DerefMut, Clone, Copy, From, Add, Ord, PartialOrd, Eq, PartialEq,
)]
pub struct Generation(pub usize);

pub fn transition_to_adult_system(
    mut commands: Commands,
    bug_query: Query<(Entity, &timers::Age, &attributes::AdultAge), With<Juvenile>>,
) {
    for (entity, age, adult_age) in bug_query.iter() {
        if age.elapsed_secs() > **adult_age {
            commands.entity(entity).remove::<Juvenile>().insert(Adult);
        }
    }
}

pub fn transition_to_hatching_system(
    mut commands: Commands,
    egg_query: Query<(Entity, &timers::Age, &attributes::HatchAge), (With<Egg>, Without<Hatching>)>,
) {
    for (entity, age, hatch_age) in egg_query.iter() {
        if age.elapsed_secs() > **hatch_age {
            commands.entity(entity).insert(Hatching);
        }
    }
}

pub fn kill_bug_system(
    mut commands: Commands,
    mut ecosystem: ResMut<ecosystem::Ecosystem>,
    mut family_tree: ResMut<ancestors::FamilyTree>,
    mut query: Query<(
        Entity,
        &mut body::Vitality,
        &attributes::DeathAge,
        &timers::Age,
        &ancestors::Relations,
    )>,
) {
    for (entity, mut vitality, death_age, age, relations) in query.iter_mut() {
        if vitality.health().amount() == 0 || **death_age < age.elapsed_secs() {
            ecosystem.return_energy(vitality.take_all_energy());
            if relations.is_interesting() {
                family_tree.dead_relations.push(relations.clone());
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}
