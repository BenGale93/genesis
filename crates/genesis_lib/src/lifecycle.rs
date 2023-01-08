use bevy::prelude::{
    AssetServer, Commands, DespawnRecursiveExt, Entity, Query, Res, ResMut, Transform, With,
    Without,
};
use genesis_attributes as attributes;
use genesis_components::*;
use genesis_traits::AttributeDisplay;

use crate::{spawning, statistics};

pub fn transition_to_adult_system(
    mut commands: Commands,
    bug_query: Query<(Entity, &time::Age, &attributes::AdultAge), With<Juvenile>>,
) {
    for (entity, age, adult_age) in bug_query.iter() {
        if age.elapsed_secs() > **adult_age {
            commands.entity(entity).remove::<Juvenile>().insert(Adult);
        }
    }
}

pub fn transition_to_hatching_system(
    mut commands: Commands,
    egg_query: Query<(Entity, &time::Age, &attributes::HatchAge), (With<Egg>, Without<Hatching>)>,
) {
    for (entity, age, hatch_age) in egg_query.iter() {
        if age.elapsed_secs() > **hatch_age {
            commands.entity(entity).insert(Hatching);
        }
    }
}

pub fn kill_bug_system(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut family_tree: ResMut<statistics::FamilyTree>,
    mut query: Query<(
        Entity,
        &mut body::Vitality,
        &attributes::DeathAge,
        &time::Age,
        &Relations,
        &Transform,
        &dyn AttributeDisplay,
    )>,
) {
    for (entity, mut vitality, death_age, age, relation, transform, attrs) in query.iter_mut() {
        if vitality.health().amount() == 0 || **death_age < age.elapsed_secs() {
            let meat_energy = vitality.take_all_energy();
            spawning::spawn_meat(
                &mut commands,
                &asset_server,
                meat_energy,
                transform.translation,
            );
            family_tree.add_dead_relation(relation, attrs);
            commands.entity(entity).despawn_recursive();
        }
    }
}
