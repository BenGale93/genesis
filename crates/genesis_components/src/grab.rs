use bevy_ecs::{prelude::Component, reflect::ReflectComponent};
use bevy_reflect::Reflect;
use bevy_time::Stopwatch;
use derive_more::{Deref, DerefMut};
use genesis_derive::BehaviourTracker;

#[derive(Component, Debug, Deref, DerefMut, Reflect, Default)]
#[reflect(Component)]
pub struct TryingToGrab(pub Stopwatch);

#[derive(Component, Debug, BehaviourTracker, Reflect, Default)]
#[reflect(Component)]
pub struct GrabbingSum(f32);

pub struct GrabComponentPlugin;

impl bevy_app::Plugin for GrabComponentPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<TryingToGrab>()
            .register_type::<GrabbingSum>();
    }
}
