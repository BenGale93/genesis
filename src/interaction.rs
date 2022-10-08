use bevy::{prelude::*, render::camera::RenderTarget};

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

pub fn get_cursor_position(
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) -> Option<Vec2> {
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width(), wnd.height());
        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        // reduce it to a 2D value
        Some(world_pos.truncate())
    } else {
        None
    }
}
