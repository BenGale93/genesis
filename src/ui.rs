use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_rapier2d::prelude::{QueryFilter, RapierContext};

use crate::{
    body,
    ecosystem::{self, Plant},
    interaction, lifecycle,
    sight::Vision,
};

pub fn energy_ui_update_system(
    mut egui_ctx: ResMut<EguiContext>,
    ecosystem: Res<ecosystem::Ecosystem>,
) {
    let energy = ecosystem.available_energy();
    egui::Window::new("Global Info")
        .anchor(egui::Align2::RIGHT_BOTTOM, [-5.0, -5.0])
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.label(format!("Global Energy: {energy}"));
        });
}

#[derive(Component)]
pub struct Selected;

pub fn select_sprite_system(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    wnds: Res<Windows>,
    mouse_button: Res<Input<MouseButton>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut sprite_query: Query<(Entity, &mut Sprite)>,
) {
    let filter = QueryFilter::default();
    if !mouse_button.pressed(MouseButton::Left) {
        return;
    }
    // check if the cursor is inside the window and get its position
    if let Some(world_pos) = interaction::get_cursor_position(wnds, q_camera) {
        for (entity, mut sprite) in sprite_query.iter_mut() {
            commands.entity(entity).remove::<Selected>();
            sprite.color = Color::WHITE;
        }
        rapier_context.intersections_with_point(world_pos, filter, |selected_entity| {
            for (entity, mut sprite) in sprite_query.iter_mut() {
                if selected_entity == entity {
                    commands.entity(selected_entity).insert(Selected);
                    sprite.color = Color::RED;
                }
            }
            false
        });
    }
}

type BugInfo<'a> = (
    &'a body::Age,
    &'a body::Vitality,
    &'a Vision,
    &'a body::InternalTimer,
    &'a lifecycle::Generation,
);

pub fn bug_info_panel_system(
    bug_query: Query<BugInfo, With<Selected>>,
    mut egui_ctx: ResMut<EguiContext>,
) {
    if let Ok(bug_info) = bug_query.get_single() {
        egui::Window::new("Bug Info")
            .anchor(egui::Align2::LEFT_TOP, [5.0, 5.0])
            .show(egui_ctx.ctx_mut(), |ui| {
                ui.label(format!("Age: {}", &bug_info.0));
                ui.label(format!("Energy: {}", &bug_info.1.energy_store()));
                ui.label(format!("Health: {}", &bug_info.1.health()));
                ui.label(format!("Visible Bugs: {}", &bug_info.2.visible_bugs()));
                ui.label(format!("Visible Food: {}", &bug_info.2.visible_food()));
                ui.label(format!("Internal timer: {}", &bug_info.3));
                ui.label(format!("Generation: {}", &bug_info.4 .0));
            });
    }
}

type EggInfo<'a> = (&'a body::Age, &'a lifecycle::Generation);

pub fn egg_info_panel_system(
    egg_query: Query<EggInfo, With<Selected>>,
    mut egui_ctx: ResMut<EguiContext>,
) {
    if let Ok(egg_info) = egg_query.get_single() {
        egui::Window::new("Egg Info")
            .anchor(egui::Align2::LEFT_TOP, [5.0, 5.0])
            .show(egui_ctx.ctx_mut(), |ui| {
                ui.label(format!("Age: {}", &egg_info.0));
                ui.label(format!("Generation: {}", &egg_info.1 .0));
            });
    }
}

type PlantInfo<'a> = &'a Plant;

pub fn plant_info_panel_system(
    plant_query: Query<PlantInfo, With<Selected>>,
    mut egui_ctx: ResMut<EguiContext>,
) {
    if let Ok(plant_info) = plant_query.get_single() {
        egui::Window::new("Plant Info")
            .anchor(egui::Align2::LEFT_TOP, [5.0, 5.0])
            .show(egui_ctx.ctx_mut(), |ui| {
                ui.label(format!("Energy: {}", &plant_info.energy()));
            });
    }
}
