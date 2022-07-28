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

// World
pub const WORLD_SIZE: f32 = 300.0;
