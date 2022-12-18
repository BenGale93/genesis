use bevy_ecs::prelude::Component;
use derive_more::Deref;

#[derive(Component)]
pub struct TryingToLay;

#[derive(Component, Copy, Clone, Debug, Deref, Ord, PartialEq, Eq, PartialOrd)]
pub struct EggsLaid(pub usize);
