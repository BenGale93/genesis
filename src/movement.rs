use bevy::{
    math::{Quat, Vec3},
    prelude::*,
};

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

pub fn move_me(me: &mut Transform, movement_factor: f32, movement_speed: f32) {
    let movement_direction = me.rotation * Vec3::Y;
    let movement_distance = movement_factor * movement_speed * config::TIME_STEP;

    me.translation += movement_direction * movement_distance;
}

pub fn movement_system(
    mut query: Query<(
        &mut Transform,
        &mind::MindOutput,
        &mut MovementSum,
        &attributes::RotationSpeed,
        &attributes::TranslationSpeed,
    )>,
) {
    for (mut transform, outputs, mut movement_sum, rotation_speed, translation_speed) in
        query.iter_mut()
    {
        let rotation_factor = outputs[config::ROTATE_INDEX].clamp(-1.0, 1.0) as f32;
        movement_sum.add_rotation(rotation_factor);
        rotate_me(&mut transform, rotation_factor, rotation_speed.value());

        let movement_factor = outputs[config::MOVEMENT_INDEX].clamp(-1.0, 1.0) as f32;
        movement_sum.add_translation(movement_factor);
        move_me(&mut transform, movement_factor, translation_speed.value());
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

    use super::{move_me, rotate_me};

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

    #[test]
    fn move_me_forwards() {
        let mut me = Transform::identity();
        let old = me;

        move_me(&mut me, 1.0, 10.0);

        assert_eq!(me.translation.x, old.translation.x);
        assert_eq!(me.translation.z, 0.0);
        assert!(me.translation.y > 0.0);
        assert_ne!(me.translation.y, old.translation.y);
    }

    #[test]
    fn move_me_backwards() {
        let mut me = Transform::identity();
        let old = me;

        move_me(&mut me, -1.0, 10.0);

        assert_eq!(me.translation.x, old.translation.x);
        assert_eq!(me.translation.z, 0.0);
        assert!(me.translation.y < 0.0);
        assert_ne!(me.translation.y, old.translation.y);
    }

    #[test]
    fn rotate_then_move() {
        let mut me = Transform::identity();

        rotate_me(&mut me, 1.0, f32::to_radians(45.0));

        move_me(&mut me, 1.0, 10.0);

        assert!(me.translation.x < 0.0);
        assert_eq!(me.translation.z, 0.0);
        assert!(me.translation.y > 0.0);
    }
}
