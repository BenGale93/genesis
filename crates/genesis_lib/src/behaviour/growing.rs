use bevy::{
    prelude::{Commands, Entity, Query, With, Without},
    sprite::Sprite,
    time::Stopwatch,
};
use bevy_rapier2d::prelude::Collider;
use genesis_attributes as attributes;
use genesis_components::{body, eat::Stomach, grow::*, mind, Egg, Size, SizeMultiplier};
use genesis_config as config;
use genesis_traits::BehaviourTracker;

use crate::spawning;

type GrowerTest<'a> = (Entity, &'a mind::MindOutput);

pub fn process_growers_system(
    mut commands: Commands,
    not_growing_query: Query<GrowerTest, (Without<Egg>, Without<TryingToGrow>)>,
    growing_query: Query<GrowerTest, With<TryingToGrow>>,
) {
    for (entity, mind_out) in not_growing_query.iter() {
        if mind_out[config::WANT_TO_GROWN_INDEX] >= 0.0 {
            commands
                .entity(entity)
                .insert(TryingToGrow(Stopwatch::new()));
        }
    }

    for (entity, mind_out) in growing_query.iter() {
        if mind_out[config::WANT_TO_GROWN_INDEX] < 0.0 {
            commands.entity(entity).remove::<TryingToGrow>();
        }
    }
}

pub fn attempted_to_grow_system(
    mut bug_query: Query<(&mut TryingToGrow, &mut GrowingSum, &attributes::GrowthRate)>,
) {
    for (mut trying_to_grow, mut grow_sum, growth_rate) in bug_query.iter_mut() {
        trying_to_grow.tick(config::BEHAVIOUR_TICK);
        let time_spent = trying_to_grow.elapsed().as_secs_f32();
        if time_spent >= 1.0 {
            grow_sum.add_time(time_spent, **growth_rate);
            trying_to_grow.reset();
        }
    }
}

pub fn grow_bug_system(
    mut grower_query: Query<
        (
            &attributes::MaxSize,
            &mut body::Vitality,
            &mut Size,
            &mut Sprite,
            &mut Collider,
            &mut GrowingSum,
            &mut SizeMultiplier,
            &mut Stomach,
        ),
        With<TryingToGrow>,
    >,
) {
    for (
        max_size,
        mut vitality,
        mut size,
        mut sprite,
        mut collider,
        mut growing_sum,
        mut size_multiplier,
        mut stomach,
    ) in grower_query.iter_mut()
    {
        let grow_amount = growing_sum.uint_portion();
        if **size >= **max_size
            || vitality.energy_store().amount()
                < grow_amount * (config::CORE_MULTIPLIER + config::HEALTH_MULTIPLIER)
        {
            continue;
        }
        size.grow(grow_amount as f32);
        vitality.grow(grow_amount, size.as_uint());
        sprite.custom_size = Some(spawning::bug_sprite_size(&size));
        *collider = spawning::bug_collider(&size);
        size_multiplier.update(**size);
        stomach.update_capacity(**size);
    }
}

pub fn existence_system(mut bug_query: Query<(&Size, &mut SizeSum)>) {
    let unit_size_cost = config::WorldConfig::global().unit_size_cost;
    for (size, mut size_sum) in bug_query.iter_mut() {
        size_sum.add_time(
            config::BEHAVIOUR_TICK.as_secs_f32(),
            **size * unit_size_cost,
        );
    }
}
