use bevy::{
    math::{Quat, Vec3},
    prelude::*,
    time::FixedTimestep,
};

use crate::{components, config};

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

pub fn movement_system(mut query: Query<(&mut Transform, &components::MindOutput)>) {
    for (mut transform, outputs) in query.iter_mut() {
        // TODO: make rotation speed an attribute of the bug.
        rotate_me(&mut transform, outputs[config::ROTATE_INDEX] as f32, 1.0);

        // TODO: make movement speed an attribute of the bug.
        move_me(&mut transform, outputs[config::MOVEMENT_INDEX] as f32, 50.0);
    }
}

pub fn movement_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(config::TIME_STEP as f64))
        .with_system(movement_system)
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
