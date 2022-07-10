use bevy::{core::FixedTimestep, prelude::*};
use rand::prelude::*;

use crate::{components, config};

fn move_bugs(mut query: Query<&mut Transform, With<components::Bug>>) {
    let mut rng = thread_rng();
    for mut transform in query.iter_mut() {
        let x_direction = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
        let y_direction = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };

        let new_x_position = transform.translation.x + x_direction * config::TIME_STEP * 100.0;
        let new_y_position = transform.translation.y + y_direction * config::TIME_STEP * 100.0;

        transform.translation.x = new_x_position;
        transform.translation.y = new_y_position;
    }
}

pub fn moving_bug_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(config::TIME_STEP as f64))
        .with_system(move_bugs)
}

fn move_camera(
    kb_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    let time_delta = time.delta().as_secs_f32();
    let (mut transform, mut projection) = camera_query.single_mut();
    // Panning.
    let mut x_direction = 0.0;
    let mut y_direction = 0.0;

    if kb_input.pressed(KeyCode::A) {
        x_direction -= 1.0;
    }
    if kb_input.pressed(KeyCode::D) {
        x_direction += 1.0;
    }

    if kb_input.pressed(KeyCode::S) {
        y_direction -= 1.0;
    }
    if kb_input.pressed(KeyCode::W) {
        y_direction += 1.0;
    }

    let new_x_position = transform.translation.x + x_direction * time_delta * config::PAN_SPEED;
    let new_y_position = transform.translation.y + y_direction * time_delta * config::PAN_SPEED;

    transform.translation.x = new_x_position;
    transform.translation.y = new_y_position;

    // Zooming
    let dist = config::ZOOM_SPEED * time_delta;
    let mut log_scale = projection.scale.ln();

    if kb_input.pressed(KeyCode::PageUp) {
        log_scale -= dist;
    }
    if kb_input.pressed(KeyCode::PageDown) {
        log_scale += dist;
    }

    projection.scale = log_scale.exp();
}

pub fn moving_camera_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(0.15))
        .with_system(move_camera)
}
