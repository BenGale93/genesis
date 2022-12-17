use bevy::{
    prelude::{Commands, Component, Deref, DerefMut, Entity, Query, Res, With, Without},
    sprite::Sprite,
    time::{Stopwatch, Time},
};
use bevy_rapier2d::prelude::Collider;

use super::lifecycle::Egg;
use crate::{attributes, body, config, mind};

#[derive(Component, Debug, Deref, DerefMut)]
pub struct TryingToGrow(pub Stopwatch);

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

#[derive(Component, Debug)]
pub struct GrowingSum(f32);

impl GrowingSum {
    pub const fn new() -> Self {
        Self(0.0)
    }

    pub fn add_growing_time(&mut self, time: f32, rate: f32) {
        self.0 += time * rate;
    }

    pub fn uint_portion(&mut self) -> usize {
        let growing_floor = self.0.floor();
        self.0 -= growing_floor;

        growing_floor as usize
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

#[derive(Component, Debug)]
pub struct SizeSum(f32);

impl SizeSum {
    pub const fn new() -> Self {
        Self(0.0)
    }

    pub fn add_existence_time(&mut self, time: f32, cost: f32) {
        self.0 += time * cost;
    }

    pub fn uint_portion(&mut self) -> usize {
        let size_floor = self.0.floor();
        self.0 -= size_floor;

        size_floor as usize
    }
}

pub fn existence_system(time: Res<Time>, mut bug_query: Query<(&body::Vitality, &mut SizeSum)>) {
    for (vitality, mut size_sum) in bug_query.iter_mut() {
        let rate = vitality.size().current_size() * config::WorldConfig::global().unit_size_cost;
        size_sum.add_existence_time(time.delta_seconds(), rate);
    }
}
