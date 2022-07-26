use bevy::{
    math::{Quat, Vec3},
    prelude::Transform,
};

use crate::config::TIME_STEP;

pub fn rotate_me(me: Transform, rotation_factor: f32, rotation_speed: f32) -> Quat {
    let current_z = me.rotation.z;
    let rotation_factor = rotation_factor.clamp(-1.0, 1.0);

    Quat::from_rotation_z(rotation_factor * rotation_speed * TIME_STEP + current_z)
}

pub fn accelerate_me(me: Transform, movement_factor: f32, movement_speed: f32) -> Vec3 {
    let movement_factor = movement_factor.clamp(-1.0, 1.0);
    let movement_direction = me.rotation * Vec3::Y;
    let movement_distance = movement_factor * movement_speed * TIME_STEP;

    movement_direction * movement_distance
}

#[cfg(test)]
mod tests {

    use bevy::{math::Quat, prelude::Transform};

    use super::{accelerate_me, rotate_me};

    #[test]
    fn rotate_clockwise() {
        let me = Transform::from_rotation(Quat::from_rotation_z(0.0));

        let new_rotation = rotate_me(me, 1.0, f32::to_radians(45.0));

        assert!(new_rotation.z > 0.0);
    }

    #[test]
    fn rotate_anti_clockwise() {
        let me = Transform::from_rotation(Quat::from_rotation_z(0.0));

        let new_rotation = rotate_me(me, -1.0, f32::to_radians(45.0));

        assert!(new_rotation.z < 0.0);
    }

    #[test]
    fn accelerate_me_forwards() {
        let me = Transform::identity();

        let new_position = accelerate_me(me, 1.0, 10.0);

        assert_eq!(new_position.x, me.translation.x);
        assert_eq!(new_position.z, 0.0);
        assert!(new_position.y > 0.0);
    }

    #[test]
    fn accelerate_me_backwards() {
        let me = Transform::identity();

        let new_position = accelerate_me(me, -1.0, 10.0);

        assert_eq!(new_position.x, me.translation.x);
        assert_eq!(new_position.z, 0.0);
        assert!(new_position.y < 0.0);
    }

    #[test]
    fn rotate_then_accelerate() {
        let mut me = Transform::identity();

        let new_rotation = rotate_me(me, 1.0, f32::to_radians(45.0));

        me.rotation = new_rotation;

        let new_position = accelerate_me(me, 1.0, 10.0);

        assert!(new_position.x < 0.0);
        assert_eq!(new_position.z, 0.0);
        assert!(new_position.y > 0.0);
    }
}
