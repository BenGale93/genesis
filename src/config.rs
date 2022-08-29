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
pub const INITIAL_SYNAPSE_COUNT: usize = 2;
pub const CHROMOSOME_COUNT: usize = 3;
pub const CHROMOSOME_LEN: usize = 100;
pub const START_ENERGY: usize = 100;

pub const MOVEMENT_INDEX: usize = 0;
pub const ROTATE_INDEX: usize = 1;
pub const REPRODUCE_INDEX: usize = 2;
pub const EAT_INDEX: usize = 3;
pub const RESET_TIMER_INDEX: usize = 4;

// World
pub const WORLD_SIZE: f32 = 500.0;

// Food
pub const FOOD_ENERGY: usize = 10;
