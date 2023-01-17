use bevy::prelude::{Query, Transform};
use bevy_rapier2d::prelude::Velocity;
use genesis_components::{mind, time::AgeEfficiency, RotationSum, SizeMultiplier, TranslationSum};
use genesis_config as config;
use genesis_traits::BehaviourTracker;

pub fn movement_system(
    mut query: Query<(
        &Transform,
        &mut Velocity,
        &mind::MindOutput,
        &mut TranslationSum,
        &mut RotationSum,
        &SizeMultiplier,
        &AgeEfficiency,
    )>,
) {
    let world_config = config::WorldConfig::global();
    for (
        transform,
        mut velocity,
        outputs,
        mut translation_sum,
        mut rotation_sum,
        size_multiplier,
        age_efficiency,
    ) in query.iter_mut()
    {
        let rotation_factor = outputs[config::ROTATE_INDEX];
        rotation_sum.add_time(
            config::BEHAVIOUR_TICK.as_secs_f32(),
            rotation_factor * world_config.rotation_cost,
        );
        velocity.angvel = size_multiplier.as_float()
            * rotation_factor
            * world_config.max_rotation
            * **age_efficiency;

        let movement_factor = outputs[config::MOVEMENT_INDEX];
        translation_sum.add_time(
            config::BEHAVIOUR_TICK.as_secs_f32(),
            movement_factor * world_config.translation_cost,
        );
        let speed = size_multiplier.as_float()
            * movement_factor
            * world_config.max_translation
            * **age_efficiency;
        velocity.linvel = (speed * transform.local_y()).truncate();
    }
}
