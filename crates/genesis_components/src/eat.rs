use bevy_ecs::prelude::Component;
use bevy_time::Stopwatch;
use derive_more::{Deref, DerefMut};
use genesis_derive::BehaviourTracker;

#[derive(Component, Debug, Deref, DerefMut)]
pub struct TryingToEat(pub Stopwatch);

#[derive(Component, Debug, BehaviourTracker)]
pub struct EatingSum(f32);

#[derive(Component, Copy, Clone, Debug, Deref, Ord, PartialEq, Eq, PartialOrd)]
pub struct EnergyConsumed(pub usize);

#[derive(Component, Debug)]
pub struct Eaten;
