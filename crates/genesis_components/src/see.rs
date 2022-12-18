use bevy_ecs::prelude::Component;
use derive_getters::Getters;

#[derive(Component, Debug, Getters)]
pub struct Vision {
    visible_bugs: u32,
    bug_angle_score: f32,
    bug_dist_score: f32,
    visible_food: u32,
    food_angle_score: f32,
    food_dist_score: f32,
}

impl Vision {
    pub const fn new() -> Self {
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

    pub fn increment_bugs(&mut self) {
        self.visible_bugs += 1;
    }

    pub fn increment_food(&mut self) {
        self.visible_food += 1;
    }

    pub fn set_bug_angle_score(&mut self, bug_angle_score: f32) {
        if self.bug_angle_score < bug_angle_score {
            self.bug_angle_score = bug_angle_score;
        }
    }

    pub fn set_bug_dist_score(&mut self, bug_dist_score: f32) {
        if self.bug_dist_score < bug_dist_score {
            self.bug_dist_score = bug_dist_score;
        }
    }

    pub fn set_visible_food(&mut self, visible_food: u32) {
        self.visible_food = visible_food;
    }

    pub fn set_food_angle_score(&mut self, food_angle_score: f32) {
        if self.food_angle_score < food_angle_score {
            self.food_angle_score = food_angle_score;
        }
    }

    pub fn set_food_dist_score(&mut self, food_dist_score: f32) {
        if self.food_dist_score < food_dist_score {
            self.food_dist_score = food_dist_score;
        }
    }
}

impl Default for Vision {
    fn default() -> Self {
        Self::new()
    }
}
