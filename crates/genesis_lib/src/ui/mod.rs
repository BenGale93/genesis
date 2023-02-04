mod brain_panel;
mod info_panels;
mod interaction;
pub mod menus;

use bevy::prelude::{App, Plugin, SystemSet};
pub use interaction::{LoadBugEvent, SaveBugEvent, SaveSimulationEvent, Selected};
use iyes_loopless::prelude::*;

use crate::{conditions, SimState};

pub fn interaction_system_set() -> SystemSet {
    ConditionSet::new()
        .run_in_state(SimState::Simulation)
        .with_system(interaction::move_camera_system)
        .with_system(interaction::camera_zooming_system)
        .with_system(interaction::pause_key_system)
        .with_system(interaction::pause_system)
        .with_system(interaction::game_controls_widget)
        .with_system(interaction::bug_serde_widget)
        .with_system(interaction::bug_spawner_widget)
        .with_system(interaction::kill_selected_system)
        .into()
}

pub fn info_panels_system_set() -> SystemSet {
    ConditionSet::new()
        .run_in_state(SimState::Simulation)
        .with_system(info_panels::bug_live_info_system)
        .with_system(info_panels::attribute_info_system)
        .with_system(info_panels::egg_live_info_panel_system)
        .with_system(info_panels::food_info_panel_system)
        .with_system(info_panels::global_ui_update_system)
        .with_system(info_panels::bug_brain_info_system)
        .with_system(info_panels::bug_stats_info_system)
        .with_system(info_panels::energy_flow_info_system)
        .into()
}

pub fn game_time_system_set() -> SystemSet {
    ConditionSet::new()
        .run_in_state(SimState::Simulation)
        .run_if(conditions::simulation_speed_changed)
        .with_system(interaction::game_time_system)
        .into()
}

pub fn selection_system_set() -> SystemSet {
    ConditionSet::new()
        .run_if_not(conditions::using_ui)
        .run_in_state(SimState::Simulation)
        .with_system(interaction::select_sprite_system)
        .into()
}

pub fn manual_spawn_system_set() -> SystemSet {
    ConditionSet::new()
        .run_if_not(conditions::using_ui)
        .run_in_state(SimState::Simulation)
        .with_system(interaction::spawn_at_mouse)
        .into()
}

pub struct GenesisUiPlugin;

impl Plugin for GenesisUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(selection_system_set())
            .add_system_set(interaction_system_set())
            .add_system_set(manual_spawn_system_set())
            .add_system_set(game_time_system_set())
            .add_system_set(info_panels_system_set())
            .insert_resource(info_panels::EntityPanelState::default())
            .add_event::<interaction::SaveSimulationEvent>()
            .add_event::<interaction::LoadBugEvent>()
            .add_event::<interaction::SaveBugEvent>();
    }
}
