use std::f32::consts::PI;

use bevy::prelude::{Query, Transform, With};
use genesis_attributes::{EyeAngle, EyeRange};
use genesis_components::{mind::Mind, see::Vision};
use genesis_ecosystem::Plant;
use genesis_maths::{angle_between, Cone};

fn dist_angle_score(
    transform: &Transform,
    target_transform: &Transform,
    eye_range: f32,
) -> (f32, f32) {
    let dist = target_transform.translation - transform.translation;
    let dist_score = dist.length() / eye_range;
    let angle = angle_between(&transform.rotation, dist);
    let angle_score = angle / PI;
    (dist_score, angle_score)
}

pub fn process_sight_system(
    mut eye_query: Query<(&EyeRange, &EyeAngle, &Transform, &mut Vision)>,
    bug_query: Query<&Transform, With<Mind>>,
    food_query: Query<&Transform, With<Plant>>,
) {
    for (eye_range, eye_angle, transform, mut vision) in eye_query.iter_mut() {
        let cone = Cone::new(
            transform.translation,
            transform.rotation,
            **eye_angle,
            **eye_range,
        )
        .unwrap();

        vision.reset();

        for bug_transform in bug_query.iter() {
            if transform.translation == bug_transform.translation {
                continue;
            }
            if cone.is_within_cone(bug_transform.translation) {
                vision.increment_bugs();

                let scores = dist_angle_score(transform, bug_transform, **eye_range);
                vision.set_bug_score(scores);
            }
        }

        for food_transform in food_query.iter() {
            if cone.is_within_cone(food_transform.translation) {
                vision.increment_food();

                let scores = dist_angle_score(transform, food_transform, **eye_range);
                vision.set_food_score(scores);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{Quat, Transform};

    use super::dist_angle_score;

    #[test]
    fn angle_score_straight_ahead() {
        let bug = Transform::from_xyz(0.0, 0.0, 0.0);
        let target = Transform::from_xyz(0.0, 1.0, 0.0);

        let (_, angle_score) = dist_angle_score(&bug, &target, 20.0);

        assert_eq!(angle_score, 0.0);
    }

    #[test]
    fn angle_score_straight_behind() {
        let bug = Transform::from_xyz(0.0, 0.0, 0.0);
        let target = Transform::from_xyz(0.0, -1.0, 0.0);

        let (_, angle_score) = dist_angle_score(&bug, &target, 20.0);

        assert!((-1.0 - angle_score).abs() < 0.0001);
    }

    #[test]
    fn angle_score_to_the_left() {
        let bug = Transform::from_xyz(0.0, 0.0, 0.0);
        let target = Transform::from_xyz(-1.0, 0.0, 0.0);

        let (_, angle_score) = dist_angle_score(&bug, &target, 20.0);

        assert!((0.5 - angle_score).abs() < 0.0001);
    }

    #[test]
    fn angle_score_to_the_right() {
        let bug = Transform::from_xyz(0.0, 0.0, 0.0);
        let target = Transform::from_xyz(1.0, 0.0, 0.0);

        let (_, angle_score) = dist_angle_score(&bug, &target, 20.0);

        assert!(((-0.5) - angle_score).abs() < 0.0001);
    }

    #[test]
    fn angle_score_offset_bug() {
        let bug = Transform::from_rotation(Quat::from_rotation_z(f32::to_radians(-45.0)));
        let target = Transform::from_xyz(-1.0, 1.0, 0.0);

        let (_, angle_score) = dist_angle_score(&bug, &target, 20.0);

        assert!(((0.5) - angle_score).abs() < 0.0001);
    }
}
