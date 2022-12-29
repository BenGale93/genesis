use bevy_ecs::prelude::Component;
use bevy_time::Stopwatch;
use derive_more::{Deref, DerefMut};

#[derive(Component, Debug, Deref, DerefMut)]
pub struct TryingToGrab(pub Stopwatch);

#[derive(Component, Debug)]
pub struct GrabbingSum(f32);

impl GrabbingSum {
    pub const fn new() -> Self {
        Self(0.0)
    }

    pub fn add_grabbing_time(&mut self, time: f32, cost: f32) {
        self.0 += time * cost;
    }

    pub fn uint_portion(&mut self) -> usize {
        let grab_floor = self.0.floor();
        self.0 -= grab_floor;

        grab_floor as usize
    }
}
