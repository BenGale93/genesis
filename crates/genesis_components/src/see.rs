use bevy_ecs::{prelude::Component, reflect::ReflectComponent};
use bevy_reflect::Reflect;
use derive_getters::Getters;

#[derive(Component, Debug, Getters, Reflect)]
#[reflect(Component)]
pub struct Vision {
    visible_bugs: u32,
    bug_angle_score: f32,
    bug_dist_score: f32,
    visible_plant: u32,
    plant_angle_score: f32,
    plant_dist_score: f32,
    visible_meat: u32,
    meat_angle_score: f32,
    meat_dist_score: f32,
}

impl Vision {
    pub const fn new() -> Self {
        Self {
            visible_bugs: 0,
            bug_angle_score: 0.0,
            bug_dist_score: 1.0,
            visible_plant: 0,
            plant_angle_score: 0.0,
            plant_dist_score: 1.0,
            visible_meat: 0,
            meat_angle_score: 0.0,
            meat_dist_score: 1.0,
        }
    }

    pub fn reset(&mut self) {
        self.visible_bugs = 0;
        self.bug_angle_score = 0.0;
        self.bug_dist_score = 1.0;
        self.visible_plant = 0;
        self.plant_angle_score = 0.0;
        self.plant_dist_score = 1.0;
        self.visible_meat = 0;
        self.meat_angle_score = 0.0;
        self.meat_dist_score = 1.0;
    }

    pub fn increment_bugs(&mut self) {
        self.visible_bugs += 1;
    }

    pub fn increment_plant(&mut self) {
        self.visible_plant += 1;
    }

    pub fn increment_meat(&mut self) {
        self.visible_meat += 1;
    }

    pub fn set_bug_score(&mut self, bug_score: (f32, f32)) {
        if self.bug_dist_score > bug_score.0 {
            self.bug_dist_score = bug_score.0;
            self.bug_angle_score = bug_score.1;
        }
    }

    pub fn set_visible_plants(&mut self, visible_plant: u32) {
        self.visible_plant = visible_plant;
    }

    pub fn set_plant_score(&mut self, plant_score: (f32, f32)) {
        if self.plant_dist_score > plant_score.0 {
            self.plant_dist_score = plant_score.0;
            self.plant_angle_score = plant_score.1;
        }
    }

    pub fn set_visible_meat(&mut self, visible_meat: u32) {
        self.visible_meat = visible_meat;
    }

    pub fn set_meat_score(&mut self, meat_score: (f32, f32)) {
        if self.meat_dist_score > meat_score.0 {
            self.meat_dist_score = meat_score.0;
            self.meat_angle_score = meat_score.1;
        }
    }
}

impl Default for Vision {
    fn default() -> Self {
        Self::new()
    }
}
