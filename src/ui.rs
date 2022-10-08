use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::{body, ecosystem, interaction, lifecycle, mind, sight::Vision};

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

pub fn select_bug_system(
    mut commands: Commands,
    wnds: Res<Windows>,
    mouse_button: Res<Input<MouseButton>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut bug_query: Query<(Entity, &Transform, &mut Sprite), With<mind::Mind>>,
) {
    if !mouse_button.pressed(MouseButton::Left) {
        return;
    }
    // check if the cursor is inside the window and get its position
    if let Some(world_pos) = interaction::get_cursor_position(wnds, q_camera) {
        for (entity, transform, mut sprite) in bug_query.iter_mut() {
            let dist = (world_pos - transform.translation.truncate()).length();
            if dist < 9.0 {
                commands.entity(entity).insert(Selected);
                sprite.color = Color::RED;
            } else {
                commands.entity(entity).remove::<Selected>();
                sprite.color = Color::WHITE;
            }
        }
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
