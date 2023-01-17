use std::fmt;

use bevy_ecs::{
    prelude::{Component, Resource},
    reflect::ReflectComponent,
};
use bevy_reflect::Reflect;
use bevy_time::Stopwatch;
use derive_more::{Deref, DerefMut};
use genesis_attributes::DeathAge;
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

#[derive(Component, Debug, Deref, DerefMut, Reflect, Default)]
#[reflect(Component)]
pub struct AgeEfficiency(pub f32);

impl AgeEfficiency {
    pub fn update(&mut self, age: &Age, death_age: &DeathAge) {
        let life_progress = age.elapsed_secs() / **death_age;

        self.0 = (1.8 - life_progress).clamp(0.0, 1.0);
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
            .register_type::<AgeEfficiency>()
            .register_type::<Heart>()
            .register_type::<InternalTimer>();
    }
}
