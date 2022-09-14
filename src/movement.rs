use bevy::{math::Quat, prelude::*};
use bevy_rapier2d::prelude::Velocity;

use crate::{attributes, body, config, mind};

#[derive(Component, Debug)]
pub struct MovementSum {
    translation_sum: f32,
    rotation_sum: f32,
}

impl MovementSum {
    pub fn new() -> Self {
        Self {
            translation_sum: 0.0,
            rotation_sum: 0.0,
        }
    }

    fn uint_portion(&mut self) -> usize {
        let tran_floor = self.translation_sum.floor();
        self.translation_sum -= tran_floor;

        let rot_floor = self.rotation_sum.floor();
        self.rotation_sum -= rot_floor;

        (tran_floor + rot_floor) as usize
    }

    fn add_translation(&mut self, translation: f32) {
        self.translation_sum += translation.abs() * config::ROTATION_COST
    }
    fn add_rotation(&mut self, rotation: f32) {
        self.rotation_sum += rotation.abs() * config::TRANSLATION_COST
    }
}

pub fn rotate_me(me: &mut Transform, rotation_factor: f32, rotation_speed: f32) {
    let z_adjustment = rotation_factor * rotation_speed * config::TIME_STEP;

    me.rotation *= Quat::from_rotation_z(z_adjustment);
}

pub fn movement_system(
    mut query: Query<(
        &mut Transform,
        &mut Velocity,
        &mind::MindOutput,
        &mut MovementSum,
        &attributes::MaxRotationRate,
        &attributes::MaxSpeed,
    )>,
) {
    for (mut transform, mut velocity, outputs, mut movement_sum, max_rotation, max_speed) in
        query.iter_mut()
    {
        let rotation_factor = outputs[config::ROTATE_INDEX].clamp(-1.0, 1.0) as f32;
        movement_sum.add_rotation(rotation_factor);
        rotate_me(&mut transform, rotation_factor, max_rotation.value());

        let movement_factor = outputs[config::MOVEMENT_INDEX].clamp(-1.0, 1.0) as f32;
        movement_sum.add_translation(movement_factor);
        let speed = movement_factor * max_speed.value();
        velocity.linvel = (speed * transform.local_y()).truncate();
    }
}

pub fn movement_energy_burn_system(
    mut query: Query<(
        &mut body::Vitality,
        &mut MovementSum,
        &mut body::BurntEnergy,
    )>,
) {
    for (mut vitality, mut movement_sum, mut burnt_energy) in query.iter_mut() {
        let energy = vitality.take_energy(movement_sum.uint_portion());
        burnt_energy.add_energy(energy)
    }
}

#[cfg(test)]
mod tests {

    use bevy::{math::Quat, prelude::Transform};

    use super::rotate_me;

    #[test]
    fn rotate_clockwise() {
        let mut me = Transform::from_rotation(Quat::from_rotation_z(0.0));

        rotate_me(&mut me, 1.0, f32::to_radians(45.0));

        assert!(me.rotation.z > 0.0);
    }

    #[test]
    fn rotate_anti_clockwise() {
        let mut me = Transform::from_rotation(Quat::from_rotation_z(0.0));

        rotate_me(&mut me, -1.0, f32::to_radians(45.0));

        assert!(me.rotation.z < 0.0);
    }
}
