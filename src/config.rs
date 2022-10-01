use bevy::prelude::{ClearColor, Color};

pub const BACKGROUND: ClearColor = ClearColor(Color::rgb(0.004, 0.09, 0.15));

pub const TIME_STEP: f32 = 1.0 / 60.0;

// Camera
pub const PAN_SPEED: f32 = 1000.0;
pub const ZOOM_SPEED: f32 = 2.0;

// Bugs
pub const INPUT_NEURONS: usize = 14;
pub const OUTPUT_NEURONS: usize = 5;
pub const START_NUM: usize = 10;
pub const INITIAL_SYNAPSE_COUNT: usize = 3;
pub const CHROMOSOME_COUNT: usize = 3;
pub const CHROMOSOME_LEN: usize = 100;

// Bugs energy
pub const START_ENERGY: usize = 1000;
pub const EATING_COST: usize = 10;
pub const ROTATION_COST: f32 = 1.0 / 10.0;
pub const TRANSLATION_COST: f32 = 1.0 / 10.0;

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

// World
pub const WORLD_SIZE: f32 = 500.0;
pub const WORLD_ENERGY: usize = 30000;

// Plant
pub const PLANT_ENERGY: usize = 100;
