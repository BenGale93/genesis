use bevy::prelude::{Component, Query, Transform};
use bevy_rapier2d::prelude::Velocity;

use crate::{attributes, config, mind};

#[derive(Component, Debug)]
pub struct MovementSum {
    translation_sum: f32,
    rotation_sum: f32,
}

impl MovementSum {
    pub const fn new() -> Self {
        Self {
            translation_sum: 0.0,
            rotation_sum: 0.0,
        }
    }

    pub fn uint_portion(&mut self) -> usize {
        let tran_floor = self.translation_sum.floor();
        self.translation_sum -= tran_floor;

        let rot_floor = self.rotation_sum.floor();
        self.rotation_sum -= rot_floor;

        (tran_floor + rot_floor) as usize
    }

    fn add_translation(&mut self, translation: f32, translation_cost: f32) {
        self.translation_sum += translation.abs() * translation_cost
    }
    fn add_rotation(&mut self, rotation: f32, rotation_cost: f32) {
        self.rotation_sum += rotation.abs() * rotation_cost
    }
}

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
        let rotation_factor = outputs[config::ROTATE_INDEX].clamp(-1.0, 1.0) as f32;
        movement_sum.add_rotation(rotation_factor, max_rotation.cost());
        velocity.angvel = rotation_factor * max_rotation.value();

        let movement_factor = outputs[config::MOVEMENT_INDEX].clamp(-1.0, 1.0) as f32;
        movement_sum.add_translation(movement_factor, max_speed.cost());
        let speed = movement_factor * max_speed.value();
        velocity.linvel = (speed * transform.local_y()).truncate();
    }
}
