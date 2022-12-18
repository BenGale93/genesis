use bevy::prelude::{Query, Transform};
use bevy_rapier2d::prelude::Velocity;
use genesis_attributes as attributes;
use genesis_components::{mind, MovementSum};
use genesis_config as config;

pub fn movement_system(
    mut query: Query<(
        &Transform,
        &mut Velocity,
        &mind::MindOutput,
        &mut MovementSum,
        &attributes::MaxRotationRate,
        &attributes::MaxSpeed,
    )>,
) {
    for (transform, mut velocity, outputs, mut movement_sum, max_rotation, max_speed) in
        query.iter_mut()
    {
        let rotation_factor = outputs[config::ROTATE_INDEX].clamp(-1.0, 1.0);
        movement_sum.add_rotation(rotation_factor, max_rotation.cost());
        velocity.angvel = rotation_factor * max_rotation.value();

        let movement_factor = outputs[config::MOVEMENT_INDEX].clamp(-1.0, 1.0);
        movement_sum.add_translation(movement_factor, max_speed.cost());
        let speed = movement_factor * max_speed.value();
        velocity.linvel = (speed * transform.local_y()).truncate();
    }
}
