use bevy::{
    math::{Quat, Vec3},
    prelude::*,
};

use crate::{body, config, mind};

pub fn rotate_me(me: &mut Transform, rotation_factor: f32, rotation_speed: f32) {
    let rotation_factor = rotation_factor.clamp(-1.0, 1.0);

    let z_adjustment = rotation_factor * rotation_speed * config::TIME_STEP;

    me.rotation *= Quat::from_rotation_z(z_adjustment);
}

pub fn move_me(me: &mut Transform, movement_factor: f32, movement_speed: f32) {
    let movement_factor = movement_factor.clamp(-1.0, 1.0);
    let movement_direction = me.rotation * Vec3::Y;
    let movement_distance = movement_factor * movement_speed * config::TIME_STEP;

    me.translation += movement_direction * movement_distance;
}

pub fn movement_system(mut query: Query<(&mut Transform, &mind::MindOutput, &body::BugBody)>) {
    for (mut transform, outputs, body) in query.iter_mut() {
        let rotation_speed = body.rotate_speed();
        rotate_me(
            &mut transform,
            outputs[config::ROTATE_INDEX] as f32,
            rotation_speed,
        );

        let movement_speed = body.movement_speed();
        move_me(
            &mut transform,
            outputs[config::MOVEMENT_INDEX] as f32,
            movement_speed,
        );
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
