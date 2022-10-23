mod gui;
mod interaction;

use bevy::{prelude::SystemSet, time::FixedTimestep};
pub use gui::PanelState;

use crate::config;

pub fn interaction_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(config::TIME_STEP as f64))
        .with_system(interaction::move_camera_system)
        .with_system(gui::bug_live_info_system)
        .with_system(gui::bug_attribute_info_system)
        .with_system(gui::egg_live_info_panel_system)
        .with_system(gui::egg_attribute_info_panel_system)
        .with_system(gui::plant_info_panel_system)
        .with_system(interaction::camera_zooming_system)
        .with_system(gui::energy_ui_update_system)
}

pub fn selection_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(gui::run_if_not_using_egui)
        .with_system(gui::select_sprite_system)
}
