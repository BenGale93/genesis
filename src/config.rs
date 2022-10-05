use std::ops::RangeInclusive;

use bevy::prelude::{ClearColor, Color};
use once_cell::sync::OnceCell;
use serde_derive::{Deserialize, Serialize};

pub const BACKGROUND: ClearColor = ClearColor(Color::rgb(0.004, 0.09, 0.15));

pub const TIME_STEP: f32 = 1.0 / 60.0;

// Camera
pub const PAN_SPEED: f32 = 1000.0;
pub const ZOOM_SPEED: f32 = 2.0;

// Bugs
pub const INPUT_NEURONS: usize = 14;
pub const OUTPUT_NEURONS: usize = 5;
pub const CHROMOSOME_COUNT: usize = 3;
pub const CHROMOSOME_LEN: usize = 100;

// Outputs
pub const MOVEMENT_INDEX: usize = 0;
pub const ROTATE_INDEX: usize = 1;
pub const REPRODUCE_INDEX: usize = 2;
pub const EAT_INDEX: usize = 3;
pub const RESET_TIMER_INDEX: usize = 4;

// Inputs
pub const CONSTANT_INDEX: usize = 0;
pub const PREV_MOVEMENT_INDEX: usize = 1;
pub const PREV_ROTATE_INDEX: usize = 2;
pub const ENERGY_INDEX: usize = 3;
pub const HEALTH_INDEX: usize = 4;
pub const AGE_INDEX: usize = 5;
pub const VISIBLE_BUGS_INDEX: usize = 6;
pub const BUG_ANGLE_SCORE_INDEX: usize = 7;
pub const BUG_DIST_SCORE_INDEX: usize = 8;
pub const VISIBLE_FOOD_INDEX: usize = 9;
pub const FOOD_ANGLE_SCORE_INDEX: usize = 10;
pub const FOOD_DIST_SCORE_INDEX: usize = 11;
pub const HEARTBEAT_INDEX: usize = 12;
pub const INTERNAL_TIMER_INDEX: usize = 13;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldConfig {
    pub start_num: usize,
    pub initial_synapse_count: usize,
    pub start_energy: usize,
    pub core_energy: usize,
    pub health_energy: usize,
    pub eating_cost: usize,
    pub rotation_cost: f32,
    pub translation_cost: f32,
    pub world_size: f32,
    pub world_energy: usize,
    pub plant_energy: usize,
}

impl WorldConfig {
    pub fn global() -> &'static WorldConfig {
        WORLD_CONFIG_INSTANCE
            .get()
            .expect("World config is not initialized")
    }

    pub fn from_config() -> Self {
        confy::load_path("./assets/config/genesis.toml").unwrap()
    }

    pub fn world_size_range(&self) -> RangeInclusive<f32> {
        -self.world_size..=self.world_size
    }
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            start_num: 10,
            initial_synapse_count: 3,
            start_energy: 1000,
            core_energy: 100,
            health_energy: 200,
            eating_cost: 10,
            rotation_cost: 0.1,
            translation_cost: 0.1,
            world_size: 1000.0,
            world_energy: 30000,
            plant_energy: 100,
        }
    }
}

pub static WORLD_CONFIG_INSTANCE: OnceCell<WorldConfig> = OnceCell::new();
