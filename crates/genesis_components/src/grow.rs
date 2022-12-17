use bevy::{prelude::Component, time::Stopwatch};
use derive_more::{Deref, DerefMut};

#[derive(Component, Debug, Deref, DerefMut)]
pub struct TryingToGrow(pub Stopwatch);

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
