use bevy_ecs::prelude::{Component, Entity};
use bevy_time::Stopwatch;
use derive_more::{Deref, DerefMut};

#[derive(Component, Debug, Deref, DerefMut)]
pub struct TryingToEat(pub Stopwatch);

#[derive(Component, Debug)]
pub struct EatingSum(f32);

impl EatingSum {
    pub const fn new() -> Self {
        Self(0.0)
    }

    pub fn add_eating_time(&mut self, time: f32, cost: f32) {
        self.0 += time * cost;
    }

    pub fn uint_portion(&mut self) -> usize {
        let eating_floor = self.0.floor();
        self.0 -= eating_floor;

        eating_floor as usize
    }
}

#[derive(Component, Copy, Clone, Debug, Deref, Ord, PartialEq, Eq, PartialOrd)]
pub struct EnergyConsumed(pub usize);

#[derive(Component, Debug)]
pub struct Eaten;

#[derive(Debug)]
pub struct EatenEvent(pub Entity);
