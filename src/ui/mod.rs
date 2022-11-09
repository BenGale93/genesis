mod brain_panel;
mod gui;
mod interaction;
mod statistics;

use std::fs;

use bevy::{
    app::AppExit,
    prelude::{EventReader, Res, SystemSet},
    time::FixedTimestep,
};
pub use gui::PanelState;
pub use statistics::GlobalStatistics;

pub fn interaction_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(interaction::move_camera_system)
        .with_system(gui::bug_live_info_system)
        .with_system(gui::bug_attribute_info_system)
        .with_system(gui::egg_live_info_panel_system)
        .with_system(gui::egg_attribute_info_panel_system)
        .with_system(gui::plant_info_panel_system)
        .with_system(interaction::camera_zooming_system)
        .with_system(gui::global_ui_update_system)
        .with_system(gui::bug_brain_info_system)
}

pub fn selection_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(gui::run_if_not_using_egui)
        .with_system(gui::select_sprite_system)
}

pub fn global_statistics_system_set() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(0.1))
        .with_system(statistics::count_system)
        .with_system(statistics::max_generation_system)
        .with_system(statistics::energy_stats_system)
        .with_system(statistics::time_elapsed_system)
}

pub fn save_on_close(events: EventReader<AppExit>, global_stats: Res<GlobalStatistics>) {
    if !events.is_empty() {
        let j = serde_json::to_string_pretty(global_stats.as_ref()).unwrap();
        fs::write("./run_data.json", j).expect("Unable to write file");
    }
}
