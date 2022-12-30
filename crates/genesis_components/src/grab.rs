use bevy_ecs::prelude::Component;
use bevy_time::Stopwatch;
use derive_more::{Deref, DerefMut};
use genesis_derive::BehaviourTracker;

#[derive(Component, Debug, Deref, DerefMut)]
pub struct TryingToGrab(pub Stopwatch);

#[derive(Component, Debug, BehaviourTracker)]
pub struct GrabbingSum(f32);
