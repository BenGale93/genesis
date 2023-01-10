use bevy_ecs::{
    prelude::{Component, Entity},
    reflect::ReflectComponent,
};
use bevy_reflect::Reflect;
use bevy_time::Stopwatch;
use derive_getters::Getters;
use derive_more::{Deref, DerefMut};
use genesis_derive::BehaviourTracker;

#[derive(Component, Debug, Deref, DerefMut, Reflect, Default)]
#[reflect(Component)]
pub struct TryingToEat(pub Stopwatch);

#[derive(Component, Debug, BehaviourTracker, Reflect, Default, Getters)]
#[reflect(Component)]
pub struct EatingSum {
    sum: f32,
    rate: f32,
}

#[derive(
    Component, Copy, Clone, Debug, Deref, Ord, PartialEq, Eq, PartialOrd, Reflect, Default,
)]
#[reflect(Component)]
pub struct EnergyConsumed(pub usize);

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Eaten;

#[derive(Debug)]
pub struct EatenEvent(pub Entity);

pub struct EatComponentPlugin;

impl bevy_app::Plugin for EatComponentPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<TryingToEat>()
            .register_type::<EatingSum>()
            .register_type::<EnergyConsumed>()
            .register_type::<Eaten>();
    }
}
