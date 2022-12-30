use bevy_ecs::prelude::Component;
use bevy_time::Stopwatch;
use derive_more::{Deref, DerefMut};
use genesis_derive::BehaviourTracker;

#[derive(Component, Debug, Deref, DerefMut)]
pub struct TryingToLay(pub Stopwatch);

#[derive(Component, Debug, BehaviourTracker)]
pub struct LayingSum(f32);

#[derive(Component, Copy, Clone, Debug, Deref, Ord, PartialEq, Eq, PartialOrd)]
pub struct EggsLaid(pub usize);
