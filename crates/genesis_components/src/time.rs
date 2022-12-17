use std::fmt;

use bevy::{
    prelude::{Component, Resource},
    time::Stopwatch,
};
use derive_more::{Deref, DerefMut};

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct SimulationTime(pub Stopwatch);

impl Default for SimulationTime {
    fn default() -> Self {
        Self(Stopwatch::new())
    }
}

impl fmt::Display for SimulationTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.elapsed().as_secs())
    }
}

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

impl Default for Heart {
    fn default() -> Self {
        Self::new()
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
