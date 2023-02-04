use std::time::Duration;

use bevy::{
    input::mouse::MouseWheel,
    prelude::{
        info, AssetServer, Camera, Color, Commands, Component, Entity, EventReader, EventWriter,
        GlobalTransform, Input, KeyCode, MouseButton, OrthographicProjection, Query,
        ReflectComponent, Res, ResMut, Transform, Vec2, Vec3, With,
    },
    reflect::Reflect,
    render::camera::RenderTarget,
    sprite::Sprite,
    time::Time,
    window::Windows,
};
use bevy_egui::{egui, EguiContext};
use bevy_rapier2d::prelude::{QueryFilter, RapierConfiguration, RapierContext, TimestepMode};
use config::WorldConfig;
use genesis_attributes as attributes;
use genesis_components::{body, time};
use genesis_config as config;
use genesis_ecosystem::Ecosystem;
use iyes_loopless::prelude::*;

use crate::{genesis_serde, simulation::SimulationSpeed, spawning, statistics};

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

pub fn pause_key_system(kb_input: Res<Input<KeyCode>>, mut speed: ResMut<SimulationSpeed>) {
    if kb_input.pressed(KeyCode::P) {
        if speed.paused {
            speed.paused = false;
        } else {
            speed.paused = true;
        };
    }
}

pub fn pause_system(speed: Res<SimulationSpeed>, mut rapier_config: ResMut<RapierConfiguration>) {
    if speed.paused {
        rapier_config.physics_pipeline_active = false;
    } else {
        rapier_config.physics_pipeline_active = true;
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Selected;

pub fn select_sprite_system(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    wnds: Res<Windows>,
    mouse_button: Res<Input<MouseButton>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut sprite_query: Query<(Entity, &mut Sprite, &body::OriginalColor)>,
) {
    let filter = QueryFilter::default();
    if !mouse_button.pressed(MouseButton::Left) {
        return;
    }
    // check if the cursor is inside the window and get its position
    let Some(world_pos) = get_cursor_position(wnds, q_camera) else {
        return;
    };
    for (entity, mut sprite, original_color) in sprite_query.iter_mut() {
        commands.entity(entity).remove::<Selected>();
        sprite.color = original_color.0;
    }
    rapier_context.intersections_with_point(world_pos, filter, |selected_entity| {
        for (entity, mut sprite, _) in sprite_query.iter_mut() {
            if selected_entity == entity {
                commands.entity(selected_entity).insert(Selected);
                sprite.color = Color::RED;
            }
        }
        false
    });
}

pub fn kill_selected_system(
    kb_input: Res<Input<KeyCode>>,
    mut ecosystem: ResMut<Ecosystem>,
    mut bug_query: Query<&mut body::Vitality, With<Selected>>,
) {
    if !kb_input.pressed(KeyCode::Delete) {
        return;
    }
    let Ok(mut vitality) = bug_query.get_single_mut() else {
        return;
    };
    let energy_extract = vitality.health_mut().take_all_energy();
    ecosystem.return_energy(energy_extract);
}

pub fn game_time_system(
    speed: Res<SimulationSpeed>,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut timesteps: ResMut<FixedTimesteps>,
    mut time: ResMut<Time>,
) {
    info!("Updating game time.");
    let very_slow = timesteps.get_mut("very_slow").unwrap();
    very_slow.step = Duration::from_secs_f32(config::VERY_SLOW_BEHAVIOUR_TICK_LENGTH / speed.speed);
    let slow = timesteps.get_mut("slow").unwrap();
    slow.step = Duration::from_secs_f32(config::SLOW_BEHAVIOUR_TICK_LENGTH / speed.speed);
    let standard = timesteps.get_mut("standard").unwrap();
    standard.step = Duration::from_secs_f32(config::BEHAVIOUR_TICK_LENGTH / speed.speed);

    rapier_config.timestep_mode = TimestepMode::Fixed {
        dt: speed.speed / 60.0,
        substeps: 1,
    };

    time.set_relative_speed(speed.speed);
}

pub fn game_controls_widget(
    mut egui_ctx: ResMut<EguiContext>,
    mut sim_speed: ResMut<SimulationSpeed>,
    mut save_stats: EventWriter<statistics::SaveStatsEvent>,
) {
    let symbol = if sim_speed.paused { "⏵" } else { "⏸" };
    let mut speed_copy = sim_speed.speed;
    egui::Window::new("Controls")
        .anchor(egui::Align2::RIGHT_TOP, [-5.0, 5.0])
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                if ui.button(symbol).clicked() {
                    sim_speed.paused = !sim_speed.paused;
                }
                ui.add(egui::Slider::new(&mut speed_copy, 0.1..=8.0).text("Game Speed"))
            });
            ui.horizontal(|ui| {
                if ui.button("Save Stats").clicked() {
                    save_stats.send(statistics::SaveStatsEvent);
                }
            })
        });

    if sim_speed.speed != speed_copy {
        sim_speed.speed = speed_copy;
    }
}

#[derive(Debug)]
pub struct SaveSimulationEvent;

#[derive(Debug)]
pub struct LoadBugEvent;

#[derive(Debug)]
pub struct SaveBugEvent;

pub fn bug_serde_widget(
    mut ev_save_sim: EventWriter<SaveSimulationEvent>,
    mut ev_load_bug: EventWriter<LoadBugEvent>,
    mut ev_save_bug: EventWriter<SaveBugEvent>,
    mut egui_ctx: ResMut<EguiContext>,
    bug_query: Query<Entity, (With<time::Age>, With<Selected>)>,
) {
    egui::Window::new("Save/Load")
        .anchor(egui::Align2::LEFT_BOTTOM, [5.0, -5.0])
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                if ui.button("Save simulation").clicked() {
                    ev_save_sim.send(SaveSimulationEvent);
                };
                if ui.button("Load bug").clicked() {
                    ev_load_bug.send(LoadBugEvent);
                };
                if bug_query.get_single().is_ok() && ui.button("Save bug").clicked() {
                    ev_save_bug.send(SaveBugEvent);
                }
            })
        });
}

pub fn bug_spawner_widget(
    mut egui_ctx: ResMut<EguiContext>,
    mut loaded_blueprint: ResMut<genesis_serde::LoadedBlueprint>,
) {
    if loaded_blueprint.blueprint.is_none() {
        return;
    }
    egui::Window::new("Spawn")
        .anchor(egui::Align2::CENTER_BOTTOM, [5.0, 0.0])
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                if ui.button("Clear Bug").clicked() {
                    loaded_blueprint.blueprint = None;
                }
            })
        });
}

pub fn spawn_at_mouse(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    genome: Res<attributes::Genome>,
    mut ecosystem: ResMut<Ecosystem>,
    loaded_blueprint: ResMut<genesis_serde::LoadedBlueprint>,
    wnds: Res<Windows>,
    mouse_button: Res<Input<MouseButton>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    if !mouse_button.just_released(MouseButton::Left) {
        return;
    }
    let Some(world_pos) = get_cursor_position(wnds, q_camera) else {
        return;
    };
    let Some(blueprint) = &loaded_blueprint.blueprint else {
        return;
    };
    let Some(energy) = ecosystem.request_energy(WorldConfig::global().start_energy) else {
        return;
    };

    spawning::spawn_egg(
        &mut commands,
        &asset_server,
        &genome,
        energy,
        Vec3::new(world_pos.x, world_pos.y, 0.0),
        blueprint.dna().to_owned(),
        blueprint.mind().to_owned(),
        genesis_components::Generation(0),
        None,
    );
}
