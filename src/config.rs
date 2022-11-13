use std::ops::RangeInclusive;

use bevy::prelude::{ClearColor, Color};
use once_cell::sync::OnceCell;
use serde_derive::{Deserialize, Serialize};

pub const BACKGROUND: ClearColor = ClearColor(Color::rgb(0.004, 0.09, 0.15));

pub const TIME_STEP: f32 = 1.0 / 60.0;

// Camera
pub const PAN_SPEED: f32 = 200.0;
pub const ZOOM_SPEED: f32 = 0.1;

// Bugs
pub const INPUT_NEURONS: usize = 14;
pub const OUTPUT_NEURONS: usize = 5;
pub const CHROMOSOME_COUNT: usize = 20;
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

type MinMaxLen = (f32, f32, usize);

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributeConfig {
    pub hatch_age: MinMaxLen,
    pub adult_age: MinMaxLen,
    pub death_age: MinMaxLen,
    pub mutation_probability: MinMaxLen,
    pub max_speed: MinMaxLen,
    pub max_rotation: MinMaxLen,
    pub eye_range: MinMaxLen,
    pub eye_angle: MinMaxLen,
    pub internal_timer_boundary: MinMaxLen,
    pub lay_egg_boundary: MinMaxLen,
    pub eating_boundary: MinMaxLen,
    pub cost_of_thought: MinMaxLen,
    pub cost_of_eating: MinMaxLen,
    pub offspring_energy: MinMaxLen,
    pub hatch_size: MinMaxLen,
}

impl Default for AttributeConfig {
    fn default() -> Self {
        Self {
            hatch_age: (30.0, 60.0, 15),
            adult_age: (50.0, 70.0, 20),
            death_age: (600.0, 700.0, 50),
            mutation_probability: (0.01, 0.35, 100),
            max_speed: (100.0, 500.0, 100),
            max_rotation: (10.0, 30.0, 20),
            eye_range: (200.0, 700.0, 100),
            eye_angle: (360.0, 30.0, 100),
            internal_timer_boundary: (-0.5, 0.5, 20),
            lay_egg_boundary: (0.0, 0.8, 30),
            eating_boundary: (-0.5, 0.5, 20),
            cost_of_thought: (0.002, 0.005, 10),
            cost_of_eating: (0.5, 0.4, 10),
            offspring_energy: (400.0, 600.0, 50),
            hatch_size: (20.0, 35.0, 15),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldConfig {
    pub start_num: usize,
    pub initial_synapse_count: usize,
    pub mutations: usize,
    pub start_energy: usize,
    pub rotation_cost: (f32, f32),
    pub translation_cost: (f32, f32),
    pub world_size: f32,
    pub world_energy: usize,
    pub plant_energy: usize,
    pub attributes: AttributeConfig,
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
            mutations: 3,
            start_energy: 800,
            rotation_cost: (0.05, 0.15),
            translation_cost: (0.05, 0.15),
            world_size: 1000.0,
            world_energy: 30000,
            plant_energy: 100,
            attributes: AttributeConfig::default(),
        }
    }
}

pub static WORLD_CONFIG_INSTANCE: OnceCell<WorldConfig> = OnceCell::new();

pub fn initialize_config() {
    let config = WorldConfig::from_config();
    _ = WORLD_CONFIG_INSTANCE.set(config);
}
