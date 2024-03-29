use bevy::prelude::{Query, Res, Transform};
use bevy_rapier2d::prelude::Velocity;
use genesis_components::{
    body::HealthEfficiency, mind, time::AgeEfficiency, RotationSum, SizeMultiplier, TranslationSum,
};
use genesis_config as config;
use genesis_traits::BehaviourTracker;
use iyes_loopless::prelude::FixedTimesteps;

pub fn movement_system(
    timesteps: Res<FixedTimesteps>,
    mut query: Query<(
        &Transform,
        &mut Velocity,
        &mind::MindOutput,
        &mut TranslationSum,
        &mut RotationSum,
        &SizeMultiplier,
        &AgeEfficiency,
        &HealthEfficiency,
    )>,
) {
    let world_config = config::WorldConfig::global();
    let standard = timesteps.get("standard").unwrap();

    for (
        transform,
        mut velocity,
        outputs,
        mut translation_sum,
        mut rotation_sum,
        size_multiplier,
        age_efficiency,
        health_efficiency,
    ) in query.iter_mut()
    {
        let rotation_factor = outputs[config::ROTATE_INDEX];
        rotation_sum.add_time(
            standard.step.as_secs_f32(),
            rotation_factor * world_config.rotation_cost,
        );
        velocity.angvel = size_multiplier.as_float()
            * rotation_factor
            * world_config.max_rotation
            * **health_efficiency
            * **age_efficiency;

        let movement_factor = outputs[config::MOVEMENT_INDEX];
        translation_sum.add_time(
            standard.step.as_secs_f32(),
            movement_factor * world_config.translation_cost,
        );
        let speed = size_multiplier.as_float()
            * movement_factor
            * world_config.max_translation
            * **health_efficiency
            * **age_efficiency;
        velocity.linvel = (speed * transform.local_y()).truncate();
    }
}
