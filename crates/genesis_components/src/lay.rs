use bevy_ecs::{prelude::Component, reflect::ReflectComponent};
use bevy_reflect::Reflect;
use bevy_time::Stopwatch;
use derive_more::{Deref, DerefMut};
use genesis_derive::BehaviourTracker;

#[derive(Component, Debug, Deref, DerefMut, Reflect, Default)]
#[reflect(Component)]
pub struct TryingToLay(pub Stopwatch);

#[derive(Component, Debug, BehaviourTracker, Reflect, Default)]
#[reflect(Component)]
pub struct LayingSum(f32);

#[derive(
    Component, Copy, Clone, Debug, Deref, Ord, PartialEq, Eq, PartialOrd, Reflect, Default,
)]
#[reflect(Component)]
pub struct EggsLaid(pub usize);

pub struct LayComponentPlugin;

impl bevy_app::Plugin for LayComponentPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<TryingToLay>()
            .register_type::<LayingSum>()
            .register_type::<EggsLaid>();
    }
}
