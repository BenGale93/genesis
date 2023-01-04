use std::{fs, io::Write};

use bevy::{
    prelude::{
        info, warn, App, AppTypeRegistry, Commands, Entity, EventReader, Plugin, Query, ResMut,
        SystemSet, With, World,
    },
    scene::DynamicScene,
    tasks::IoTaskPool,
    time::Time,
};
use bevy_egui::{egui, EguiContext};
use genesis_components::body::OriginalColor;
use iyes_loopless::prelude::*;

use super::gui;
use crate::{genesis_serde, SimState};

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
    ev_save_sim: EventReader<gui::SaveSimulationEvent>,
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

fn save_simulation_system(world: &World) {
    let type_registry = world.resource::<AppTypeRegistry>();
    let scene = DynamicScene::from_world(world, type_registry);
    let serialized_scene = scene.serialize_ron(type_registry).unwrap();
    let serialized_sim = genesis_serde::serialize_simulation(world);
    let path = std::env::current_dir().unwrap();
    let Some(res) = rfd::FileDialog::new()
                        .set_directory(path)
                        .pick_folder() else
                    {
                        return;
                    };

    IoTaskPool::get()
        .spawn(async move {
            match fs::File::create(res.join("scene.scn.ron"))
                .and_then(|mut file| file.write(serialized_scene.as_bytes()))
            {
                Ok(_) => info!("Saved simulation."),
                Err(e) => warn!("Could not save simulation. Please try again: {e}."),
            };
            match fs::File::create(res.join("resources.ron"))
                .and_then(|mut file| file.write(serialized_sim.as_bytes()))
            {
                Ok(_) => info!("Saved simulation resources."),
                Err(e) => warn!("Could not save simulation. Please try again: {e}."),
            };
        })
        .detach();
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
            .add_enter_system(SimState::Saving, save_simulation_system)
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
