use bevy::{
    prelude::{Commands, Entity, Query, Res, With, Without},
    sprite::Sprite,
    time::{Stopwatch, Time},
};
use bevy_rapier2d::prelude::Collider;

use crate::{
    attributes, body,
    components::{grow::*, mind, Egg},
    config,
};

type GrowerTest<'a> = (
    Entity,
    &'a mind::MindOutput,
    &'a attributes::WantToGrowBoundary,
);

pub fn process_growers_system(
    mut commands: Commands,
    not_growing_query: Query<GrowerTest, (Without<Egg>, Without<TryingToGrow>)>,
    growing_query: Query<GrowerTest, With<TryingToGrow>>,
) {
    for (entity, mind_out, boundary) in not_growing_query.iter() {
        if mind_out[config::WANT_TO_GROWN_INDEX] > **boundary {
            commands
                .entity(entity)
                .insert(TryingToGrow(Stopwatch::new()));
        }
    }

    for (entity, mind_out, boundary) in growing_query.iter() {
        if mind_out[config::WANT_TO_GROWN_INDEX] <= **boundary {
            commands.entity(entity).remove::<TryingToGrow>();
        }
    }
}

pub fn attempted_to_grow_system(
    time: Res<Time>,
    mut bug_query: Query<(&mut TryingToGrow, &mut GrowingSum, &attributes::GrowthRate)>,
) {
    for (mut trying_to_grow, mut grow_sum, growth_rate) in bug_query.iter_mut() {
        trying_to_grow.tick(time.delta());
        let time_spent = trying_to_grow.elapsed().as_secs_f32();
        if time_spent >= 1.0 {
            grow_sum.add_growing_time(time_spent, **growth_rate);
            trying_to_grow.reset();
        }
    }
}

pub fn grow_bug_system(
    mut grower_query: Query<
        (
            &mut body::Vitality,
            &mut Sprite,
            &mut Collider,
            &mut GrowingSum,
        ),
        With<TryingToGrow>,
    >,
) {
    for (mut vitality, mut sprite, mut collider, mut growing_sum) in grower_query.iter_mut() {
        match vitality.grow(growing_sum.uint_portion()) {
            Ok(()) => (),
            Err(_) => continue,
        };
        sprite.custom_size = Some(vitality.size().sprite());
        *collider = vitality.size().collider();
    }
}

pub fn existence_system(time: Res<Time>, mut bug_query: Query<(&body::Vitality, &mut SizeSum)>) {
    for (vitality, mut size_sum) in bug_query.iter_mut() {
        let rate = vitality.size().current_size() * config::WorldConfig::global().unit_size_cost;
        size_sum.add_existence_time(time.delta_seconds(), rate);
    }
}
