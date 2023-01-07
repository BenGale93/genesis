use bevy::{
    prelude::{info, App, Commands, Entity, EventReader, Plugin, Query, ResMut, SystemSet, With},
    time::Time,
};
use bevy_egui::{egui, EguiContext};
use genesis_components::body::OriginalColor;
use iyes_loopless::prelude::*;

use super::interaction;
use crate::SimState;

fn main_menu_system(mut egui_ctx: ResMut<EguiContext>, mut commands: Commands) {
    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.heading("Genesis Life Simulator");
        if ui.button("New simulation").clicked() {
            commands.insert_resource(NextState(SimState::Simulation));
        };
        if ui.button("Load simulation").clicked() {
            commands.insert_resource(NextState(SimState::Loading));
        }
    });
}

fn main_menu_system_set() -> SystemSet {
    ConditionSet::new()
        .run_in_state(SimState::MainMenu)
        .with_system(main_menu_system)
        .into()
}

fn transition_to_saving_system(
    mut commands: Commands,
    ev_save_sim: EventReader<interaction::SaveSimulationEvent>,
) {
    if !ev_save_sim.is_empty() {
        ev_save_sim.clear();
        commands.insert_resource(NextState(SimState::Saving));
    }
}

fn transition_to_saving_system_set() -> SystemSet {
    ConditionSet::new()
        .run_in_state(SimState::Simulation)
        .with_system(transition_to_saving_system)
        .into()
}

fn transition_to_simulation_system(mut commands: Commands, mut time: ResMut<Time>) {
    info!("Transitioning to Simulation state.");
    // Need to update time here manually otherwise we get a huge delta for the next tick.
    time.update();
    commands.insert_resource(NextState(SimState::Simulation));
}

fn check_entities_are_loaded_system(
    mut commands: Commands,
    entity_query: Query<Entity, With<OriginalColor>>,
) {
    if entity_query.iter().len() > 0 {
        info!("Transitioning to Simulation state.");
        commands.insert_resource(NextState(SimState::Simulation));
    };
}

pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(main_menu_system_set())
            .add_system_set(transition_to_saving_system_set())
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(SimState::Saving)
                    .with_system(transition_to_simulation_system)
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(SimState::Loading)
                    .with_system(check_entities_are_loaded_system)
                    .into(),
            );
    }
}
