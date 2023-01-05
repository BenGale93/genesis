use std::fmt;

use bevy_ecs::{
    prelude::{Component, Resource},
    reflect::ReflectComponent,
};
use bevy_reflect::Reflect;
use bevy_time::Stopwatch;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

#[derive(Resource, Debug, Deref, DerefMut, Serialize, Deserialize, Clone)]
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

#[derive(Component, Debug, Deref, DerefMut, Reflect)]
#[reflect(Component)]
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

#[derive(Component, Debug, Deref, DerefMut, Reflect)]
#[reflect(Component)]
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

#[derive(Component, Debug, Deref, DerefMut, Reflect)]
#[reflect(Component)]
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

pub struct TimeComponentPlugin;

impl bevy_app::Plugin for TimeComponentPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<Age>()
            .register_type::<Heart>()
            .register_type::<InternalTimer>();
    }
}
