use std::f32::consts::PI;

use bevy::prelude::{Query, Transform, With};
use genesis_attributes::{EyeAngle, EyeRange};
use genesis_components::{mind::Mind, see::Vision, time::AgeEfficiency, Meat, Plant};
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
    mut eye_query: Query<(
        &EyeRange,
        &EyeAngle,
        &Transform,
        &mut Vision,
        &AgeEfficiency,
    )>,
    bug_query: Query<&Transform, With<Mind>>,
    plant_query: Query<&Transform, With<Plant>>,
    meat_query: Query<&Transform, With<Meat>>,
) {
    for (eye_range, eye_angle, transform, mut vision, age_efficiency) in eye_query.iter_mut() {
        let range = **eye_range * **age_efficiency;
        let cone = Cone::new(
            transform.translation,
            transform.rotation,
            **eye_angle,
            range,
        )
        .unwrap();

        vision.reset();

        for bug_transform in bug_query.iter() {
            if transform.translation == bug_transform.translation {
                continue;
            }
            if cone.is_within_cone(bug_transform.translation) {
                vision.increment_bugs();

                let scores = dist_angle_score(transform, bug_transform, range);
                vision.set_bug_score(scores);
            }
        }

        for plant_transform in plant_query.iter() {
            if cone.is_within_cone(plant_transform.translation) {
                vision.increment_plant();

                let scores = dist_angle_score(transform, plant_transform, range);
                vision.set_plant_score(scores);
            }
        }

        for meat_transform in meat_query.iter() {
            if cone.is_within_cone(meat_transform.translation) {
                vision.increment_meat();

                let scores = dist_angle_score(transform, meat_transform, range);
                vision.set_meat_score(scores);
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
