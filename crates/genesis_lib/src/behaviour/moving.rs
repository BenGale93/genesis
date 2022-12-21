use bevy::prelude::{Query, Transform};
use bevy_rapier2d::prelude::Velocity;
use genesis_components::{mind, MovementSum, SizeMultiplier};
use genesis_config as config;

pub fn movement_system(
    mut query: Query<(
        &Transform,
        &mut Velocity,
        &mind::MindOutput,
        &mut MovementSum,
        &SizeMultiplier,
    )>,
) {
    let world_config = config::WorldConfig::global();
    for (transform, mut velocity, outputs, mut movement_sum, size_multiplier) in query.iter_mut() {
        let rotation_factor = outputs[config::ROTATE_INDEX];
        movement_sum.add_rotation(rotation_factor, world_config.rotation_cost);
        velocity.angvel = size_multiplier.as_float() * rotation_factor * world_config.max_rotation;

        let movement_factor = outputs[config::MOVEMENT_INDEX];
        movement_sum.add_translation(movement_factor, world_config.translation_cost);
        let speed = size_multiplier.as_float() * movement_factor * world_config.max_translation;
        velocity.linvel = (speed * transform.local_y()).truncate();
    }
}
