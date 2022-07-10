use bevy::prelude::{ClearColor, Color};

pub const BACKGROUND: ClearColor = ClearColor(Color::rgb(0.004, 0.09, 0.15));

pub const TIME_STEP: f32 = 1.0 / 60.0;

// Camera
pub const PAN_SPEED: f32 = 1000.0;
pub const ZOOM_SPEED: f32 = 1.0;
