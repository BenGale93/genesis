use bevy::prelude::*;
use genesis_util::maths::Cone;

use crate::{
    attributes::{EyeAngle, EyeRange},
    ecosystem::Plant,
    mind::Mind,
};

#[derive(Component, Debug)]
pub struct Vision {
    visible_bugs: u32,
    visible_food: u32,
}

impl Vision {
    pub fn new() -> Self {
        Self {
            visible_bugs: 0,
            visible_food: 0,
        }
    }

    pub fn visible_bugs(&self) -> u32 {
        self.visible_bugs
    }

    pub fn visible_food(&self) -> u32 {
        self.visible_food
    }
}

impl Default for Vision {
    fn default() -> Self {
        Self::new()
    }
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
            eye_angle.value(),
            eye_range.value(),
        )
        .unwrap();
        vision.visible_bugs = 0;
        for bug_transform in bug_query.iter() {
            if transform.translation == bug_transform.translation {
                continue;
            } else if cone.is_within_cone(bug_transform.translation) {
                vision.visible_bugs += 1;
            }
        }

        vision.visible_food = 0;
        for food_transform in food_query.iter() {
            if cone.is_within_cone(food_transform.translation) {
                vision.visible_food += 1;
            }
        }
    }
}
