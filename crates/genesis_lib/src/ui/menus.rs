use bevy::prelude::{App, Commands, Plugin, ResMut, SystemSet};
use bevy_egui::{egui, EguiContext};
use iyes_loopless::prelude::*;

use crate::SimState;

pub fn main_menu_system(mut egui_ctx: ResMut<EguiContext>, mut commands: Commands) {
    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.heading("Genesis Life Simulator");
        if ui.button("New simulation").clicked() {
            commands.insert_resource(NextState(SimState::Simulation));
        }
    });
}

fn main_menu_system_set() -> SystemSet {
    ConditionSet::new()
        .run_in_state(SimState::MainMenu)
        .with_system(main_menu_system)
        .into()
}

pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(main_menu_system_set());
    }
}
