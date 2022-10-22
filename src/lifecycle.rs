use bevy::prelude::*;
use derive_more::{Add, From};

use crate::{attributes, body, mind, spawn};

#[derive(Component, Debug)]
pub struct Hatching;

#[derive(Component, Debug)]
pub struct Juvenile;

#[derive(Component, Debug)]
pub struct Adult;

#[derive(Component, Debug, Deref, DerefMut, Clone, Copy, From, Add)]
pub struct Generation(pub usize);

pub fn transition_to_adult_system(
    mut commands: Commands,
    bug_query: Query<(Entity, &body::Age, &attributes::AdultAge), With<Juvenile>>,
) {
    for (entity, age, adult_age) in bug_query.iter() {
        if age.elapsed_secs() > **adult_age {
            commands.entity(entity).remove::<Juvenile>().insert(Adult);
        }
    }
}

pub fn transition_to_hatching_system(
    mut commands: Commands,
    egg_query: Query<(Entity, &body::Age, &attributes::HatchAge), Without<Hatching>>,
) {
    for (entity, age, hatch_age) in egg_query.iter() {
        if age.elapsed_secs() > **hatch_age {
            commands.entity(entity).insert(Hatching);
        }
    }
}

type Egg<'a> = (
    Entity,
    &'a mut body::Vitality,
    &'a Transform,
    &'a mind::Mind,
    &'a body::BugBody,
    &'a Generation,
);

pub fn hatch_egg_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut hatch_query: Query<Egg, With<Hatching>>,
) {
    for (entity, mut vitality, transform, mind, body, generation) in hatch_query.iter_mut() {
        commands.entity(entity).despawn();
        spawn::spawn_bug(
            &mut commands,
            &asset_server,
            vitality.move_all_energy(),
            (body.clone(), mind.clone(), transform, *generation),
        )
    }
}
