use std::f32::consts::PI;

use bevy::prelude::*;
use genesis_util::maths;

use crate::{
    attributes::{EyeAngle, EyeRange},
    ecosystem::Plant,
    mind::Mind,
};

#[derive(Component, Debug)]
pub struct Vision {
    visible_bugs: u32,
    bug_angle_score: f32,
    bug_dist_score: f32,
    visible_food: u32,
    food_angle_score: f32,
    food_dist_score: f32,
}

impl Vision {
    pub fn new() -> Self {
        Self {
            visible_bugs: 0,
            bug_angle_score: 0.0,
            bug_dist_score: 0.0,
            visible_food: 0,
            food_angle_score: 0.0,
            food_dist_score: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.visible_bugs = 0;
        self.bug_angle_score = 0.0;
        self.bug_dist_score = 0.0;
        self.visible_food = 0;
        self.food_angle_score = 0.0;
        self.food_dist_score = 0.0;
    }

    pub fn visible_bugs(&self) -> u32 {
        self.visible_bugs
    }

    pub fn bug_angle_score(&self) -> f32 {
        self.bug_angle_score
    }

    pub fn bug_dist_score(&self) -> f32 {
        self.bug_dist_score
    }

    pub fn visible_food(&self) -> u32 {
        self.visible_food
    }

    pub fn food_angle_score(&self) -> f32 {
        self.food_angle_score
    }

    pub fn food_dist_score(&self) -> f32 {
        self.food_dist_score
    }
}

impl Default for Vision {
    fn default() -> Self {
        Self::new()
    }
}

fn dist_angle_score(transform: &Transform, target_transform: &Transform) -> (f32, f32) {
    let dist = target_transform.translation - transform.translation;
    let dist_score = 1.0 / dist.length();
    let angle_to_target = maths::angle_to_point(dist) - PI / 2.0;
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
        let cone = maths::Cone::new(
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
                vision.visible_bugs += 1;

                let (dist_score, angle_score) = dist_angle_score(transform, bug_transform);
                if vision.bug_dist_score < dist_score {
                    vision.bug_dist_score = dist_score;
                }
                if vision.bug_angle_score < angle_score {
                    vision.bug_angle_score = angle_score;
                }
            }
        }

        for food_transform in food_query.iter() {
            if cone.is_within_cone(food_transform.translation) {
                vision.visible_food += 1;

                let (dist_score, angle_score) = dist_angle_score(transform, food_transform);
                if vision.food_dist_score < dist_score {
                    vision.food_dist_score = dist_score;
                }
                if vision.food_angle_score < angle_score {
                    vision.food_angle_score = angle_score;
                }
            }
        }
    }
}
