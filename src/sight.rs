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
}

impl Vision {
    pub fn new() -> Self {
        Self { visible_bugs: 0 }
    }

    pub fn visible_bugs(&self) -> u32 {
        self.visible_bugs
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
    _food_query: Query<&Transform, With<Plant>>,
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
    }
}
