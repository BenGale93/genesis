use std::f32::consts::PI;

use bevy::prelude::{Quat, Query, Transform, With};
use genesis_attributes::{EyeAngle, EyeRange};
use genesis_components::{mind::Mind, see::Vision};
use genesis_ecosystem::Plant;
use genesis_maths::{angle_to_point, Cone};

fn dist_angle_score(transform: &Transform, target_transform: &Transform) -> (f32, f32) {
    let dist = target_transform.translation - transform.translation;
    let dist_score = 1.0 / dist.length();
    let angle_to_target = angle_to_point(dist) - PI / 2.0;
    let angle_diff = transform
        .rotation
        .angle_between(Quat::from_rotation_z(angle_to_target));
    let angle_score = (PI - angle_diff.abs()) / PI;
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
                let visible_bugs = vision.visible_bugs() + 1;
                vision.set_visible_bugs(visible_bugs);

                let (dist_score, angle_score) = dist_angle_score(transform, bug_transform);
                if vision.bug_dist_score() < &dist_score {
                    vision.set_bug_dist_score(dist_score);
                }
                if vision.bug_angle_score() < &angle_score {
                    vision.set_bug_angle_score(angle_score);
                }
            }
        }

        for food_transform in food_query.iter() {
            if cone.is_within_cone(food_transform.translation) {
                let visible_food = vision.visible_food() + 1;
                vision.set_visible_food(visible_food);

                let (dist_score, angle_score) = dist_angle_score(transform, food_transform);
                if vision.food_dist_score() < &dist_score {
                    vision.set_food_dist_score(dist_score);
                }
                if vision.food_angle_score() < &angle_score {
                    vision.set_food_angle_score(angle_score);
                }
            }
        }
    }
}
