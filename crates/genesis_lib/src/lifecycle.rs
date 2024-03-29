use bevy::{
    prelude::{
        AssetServer, Commands, DespawnRecursiveExt, Entity, EventReader, Query, Res, ResMut,
        Transform, With,
    },
    sprite::Sprite,
};
use bevy_rapier2d::prelude::Collider;
use ecosystem::EggEnergy;
use genesis_attributes as attributes;
use genesis_components::*;
use genesis_config as config;
use genesis_ecosystem as ecosystem;
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

type EggQuery<'a> = (
    Entity,
    &'a time::Age,
    &'a attributes::HatchAge,
    &'a mut ecosystem::EggEnergy,
    &'a mind::Mind,
    &'a Sprite,
    &'a attributes::HatchSize,
);

pub fn hatch_egg_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ecosystem: ResMut<ecosystem::Ecosystem>,
    mut hatch_query: Query<EggQuery, With<Egg>>,
) {
    for (entity, age, hatch_age, mut egg_energy, mind, sprite, hatch_size) in hatch_query.iter_mut()
    {
        if age.elapsed_secs() < **hatch_age {
            continue;
        }
        commands.entity(entity).remove::<spawning::EggBundle>();
        let hatching_entity = commands.entity(entity);
        let leftover_energy = spawning::spawn_bug(
            &asset_server,
            egg_energy.move_all_energy(),
            (mind.clone(), &sprite.color, hatch_size),
            hatching_entity,
        );
        ecosystem.return_energy(leftover_energy);
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

pub fn kill_egg_system(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut ev_egg: EventReader<DeadEggEvent>,
    mut egg_query: Query<(Entity, &Transform, &mut EggEnergy)>,
) {
    for ev in ev_egg.iter() {
        if let Ok(egg) = egg_query.get_mut(ev.0) {
            let (egg_entity, egg_transform, mut egg_energy) = egg;
            let meat_energy = egg_energy.move_all_energy();
            spawning::spawn_meat(
                &mut commands,
                &asset_server,
                meat_energy,
                egg_transform.translation,
            );
            commands.entity(egg_entity).despawn_recursive();
        }
    }
}

pub fn rot_meat_system(
    mut ecosystem: ResMut<ecosystem::Ecosystem>,
    mut meat_query: Query<
        (&mut Sprite, &mut Collider, &mut Size, &mut ecosystem::Food),
        With<Meat>,
    >,
) {
    let rot_rate = config::WorldConfig::global().meat.rot_rate;
    for (mut sprite, mut collider, mut size, mut meat) in meat_query.iter_mut() {
        let rotting_energy = meat.take_energy(rot_rate);
        **size = meat.size();
        sprite.custom_size = Some(spawning::food_sprite_size(&size));
        *collider = spawning::food_collider(&size);
        ecosystem.return_energy(rotting_energy);
    }
}
