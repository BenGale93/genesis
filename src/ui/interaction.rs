use std::time::Duration;

use bevy::{
    input::mouse::MouseWheel,
    prelude::{
        Camera, EventReader, GlobalTransform, Input, KeyCode, OrthographicProjection, Query, Res,
        ResMut, Resource, Transform, Vec2, Vec3, With,
    },
    render::camera::RenderTarget,
    time::Time,
    window::Windows,
};
use bevy_rapier2d::prelude::{RapierConfiguration, TimestepMode};
use iyes_loopless::prelude::*;

use crate::config;

pub fn move_camera_system(
    kb_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    let time_delta = time.delta().as_secs_f32();
    let (mut transform, _) = camera_query.single_mut();
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

    let new_x_position =
        (x_direction * time_delta).mul_add(config::PAN_SPEED, transform.translation.x);
    let new_y_position =
        (y_direction * time_delta).mul_add(config::PAN_SPEED, transform.translation.y);

    transform.translation.x = new_x_position;
    transform.translation.y = new_y_position;
}

pub fn camera_zooming_system(
    mut mouse_wheel_event_reader: EventReader<MouseWheel>,
    mut query: Query<(&Camera, &mut Transform)>,
) {
    let mut zoom_scalar = 1.0;
    for mouse_wheel_event in mouse_wheel_event_reader.iter() {
        zoom_scalar *= config::ZOOM_SPEED.mul_add(-mouse_wheel_event.y, 1.0);
    }

    for (_, mut transform) in query.iter_mut() {
        // BUG: for some reason, when camera scale < 1.0, things just disappear!
        let zoomed = transform.scale * zoom_scalar;
        let limited = Vec3::new(zoomed.x.max(1.0), zoomed.y.max(1.0), zoomed.z.max(1.0));
        transform.scale = limited;
    }
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
    wnd.cursor_position().map(|screen_pos| {
        // get the size of the window
        let window_size = Vec2::new(wnd.width(), wnd.height());
        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        // reduce it to a 2D value
        world_pos.truncate()
    })
}

#[derive(Resource, Debug)]
pub struct SimulationSpeed {
    pub speed: f32,
    pub paused: bool,
}

impl Default for SimulationSpeed {
    fn default() -> Self {
        Self {
            speed: 1.0,
            paused: false,
        }
    }
}

pub fn pause_key_system(kb_input: Res<Input<KeyCode>>, mut speed: ResMut<SimulationSpeed>) {
    if kb_input.pressed(KeyCode::P) {
        if speed.paused {
            speed.paused = false;
        } else {
            speed.paused = true;
        };
    }
}

pub fn is_paused(speed: Res<SimulationSpeed>) -> bool {
    speed.paused
}

pub fn pause_system(speed: Res<SimulationSpeed>, mut rapier_config: ResMut<RapierConfiguration>) {
    if speed.paused {
        rapier_config.physics_pipeline_active = false;
    } else {
        rapier_config.physics_pipeline_active = true;
    }
}

// TODO: This should only run if SimulationSpeed changes.
pub fn game_time_system(
    speed: Res<SimulationSpeed>,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut timesteps: ResMut<FixedTimesteps>,
    mut time: ResMut<Time>,
) {
    let very_slow = timesteps.get_mut("very_slow").unwrap();
    very_slow.step = Duration::from_secs_f32(1.0 / speed.speed);
    let slow = timesteps.get_mut("slow").unwrap();
    slow.step = Duration::from_secs_f32(0.1 / speed.speed);
    let standard = timesteps.get_mut("standard").unwrap();
    standard.step = Duration::from_secs_f32(0.05 / speed.speed);

    rapier_config.timestep_mode = TimestepMode::Fixed {
        dt: speed.speed / 60.0,
        substeps: 1,
    };

    time.set_relative_speed(speed.speed);
}
