use std::fmt;

use bevy::{
    prelude::{Component, Query, Res},
    time::{Stopwatch, Time},
};
use derive_more::{Deref, DerefMut};

use crate::{attributes, config, mind::MindOutput};

#[derive(Component, Debug, Deref, DerefMut)]
pub struct Age(pub Stopwatch);

impl Default for Age {
    fn default() -> Self {
        Self(Stopwatch::new())
    }
}

impl fmt::Display for Age {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.elapsed().as_secs())
    }
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct Heart(pub Stopwatch);

impl Heart {
    pub fn new() -> Self {
        Self(Stopwatch::new())
    }

    pub fn pulse(&self) -> f32 {
        self.elapsed_secs().sin()
    }
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct InternalTimer(pub Stopwatch);

impl InternalTimer {
    pub fn new() -> Self {
        Self(Stopwatch::new())
    }
}

impl Default for InternalTimer {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for InternalTimer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.elapsed().as_secs())
    }
}

pub fn progress_age_system(time: Res<Time>, mut query: Query<&mut Age>) {
    for mut age in query.iter_mut() {
        age.tick(time.delta());
    }
}

pub fn progress_timers_system(time: Res<Time>, mut query: Query<(&mut Heart, &mut InternalTimer)>) {
    for (mut heart, mut internal_timer) in query.iter_mut() {
        heart.tick(time.delta());
        internal_timer.tick(time.delta());
    }
}

pub fn reset_internal_timer_system(
    mut query: Query<(
        &mut InternalTimer,
        &MindOutput,
        &attributes::InternalTimerBoundary,
    )>,
) {
    for (mut internal_timer, mind_out, boundary) in query.iter_mut() {
        if mind_out[config::RESET_TIMER_INDEX] > **boundary {
            internal_timer.reset();
        }
    }
}
