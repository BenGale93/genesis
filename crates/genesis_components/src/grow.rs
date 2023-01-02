use bevy_ecs::{prelude::Component, reflect::ReflectComponent};
use bevy_reflect::Reflect;
use bevy_time::Stopwatch;
use derive_more::{Deref, DerefMut};
use genesis_derive::BehaviourTracker;

#[derive(Component, Debug, Deref, DerefMut, Reflect, Default)]
#[reflect(Component)]
pub struct TryingToGrow(pub Stopwatch);

#[derive(Component, Debug, BehaviourTracker, Reflect, Default)]
#[reflect(Component)]
pub struct GrowingSum(f32);

#[derive(Component, Debug, BehaviourTracker, Reflect, Default)]
#[reflect(Component)]
pub struct SizeSum(f32);

pub struct GrowComponentPlugin;

impl bevy_app::Plugin for GrowComponentPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<TryingToGrow>()
            .register_type::<GrowingSum>()
            .register_type::<SizeSum>();
    }
}
