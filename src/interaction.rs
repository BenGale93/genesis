use bevy::prelude::*;

use crate::config;

pub fn move_camera_system(
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
