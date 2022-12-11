mod brain_panel;
mod gui;
mod interaction;
mod statistics;

use std::{fs, time::Duration};

use bevy::{
    app::AppExit,
    prelude::{App, CoreStage, EventReader, Plugin, Res, SystemSet},
    time::Time,
};
pub use gui::{EntityPanelState, GlobalPanelState};
pub use interaction::is_paused;
use iyes_loopless::prelude::*;
use serde_derive::Serialize;
pub use statistics::{
    AverageAttributeStatistics, BugPerformanceStatistics, CountStatistics, EnergyStatistics,
};

use crate::config::WorldConfig;

#[derive(Debug, Serialize)]
struct RunInfo<'a> {
    time_elapsed: &'a f64,
    run_config: &'a WorldConfig,
    count_stats: &'a statistics::CountStatistics,
    energy_stats: &'a statistics::EnergyStatistics,
    performance_stats: &'a statistics::BugPerformanceStatistics,
    attribute_stats: &'a statistics::AverageAttributeStatistics,
}

impl<'a> RunInfo<'a> {
    const fn new(
        time_elapsed: &'a f64,
        run_config: &'a WorldConfig,
        count_stats: &'a statistics::CountStatistics,
        energy_stats: &'a statistics::EnergyStatistics,
        performance_stats: &'a statistics::BugPerformanceStatistics,
        attribute_stats: &'a statistics::AverageAttributeStatistics,
    ) -> Self {
        Self {
            time_elapsed,
            run_config,
            count_stats,
            energy_stats,
            performance_stats,
            attribute_stats,
        }
    }
}

fn save_stats(
    time: &Res<Time>,
    count_stats: &Res<statistics::CountStatistics>,
    energy_stats: &Res<statistics::EnergyStatistics>,
    performance_stats: &Res<statistics::BugPerformanceStatistics>,
    attribute_stats: &Res<statistics::AverageAttributeStatistics>,
) {
    let time = time.elapsed_seconds_f64();
    let run_info = RunInfo::new(
        &time,
        WorldConfig::global(),
        count_stats,
        energy_stats,
        performance_stats,
        attribute_stats,
    );
    let j = serde_json::to_string_pretty(&run_info).unwrap();
    fs::write("./run_data.json", j).expect("Unable to write file.");
}

pub fn save_on_close(
    events: EventReader<AppExit>,
    time: Res<Time>,
    count_stats: Res<statistics::CountStatistics>,
    energy_stats: Res<statistics::EnergyStatistics>,
    performance_stats: Res<statistics::BugPerformanceStatistics>,
    attribute_stats: Res<statistics::AverageAttributeStatistics>,
) {
    if !events.is_empty() {
        save_stats(
            &time,
            &count_stats,
            &energy_stats,
            &performance_stats,
            &attribute_stats,
        );
    }
}

pub fn regular_saver(
    time: Res<Time>,
    count_stats: Res<statistics::CountStatistics>,
    energy_stats: Res<statistics::EnergyStatistics>,
    performance_stats: Res<statistics::BugPerformanceStatistics>,
    attribute_stats: Res<statistics::AverageAttributeStatistics>,
) {
    save_stats(
        &time,
        &count_stats,
        &energy_stats,
        &performance_stats,
        &attribute_stats,
    );
}

pub fn interaction_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(interaction::move_camera_system)
        .with_system(interaction::camera_zooming_system)
        .with_system(interaction::pause_key_system)
        .with_system(interaction::pause_system)
        .with_system(interaction::game_time_system)
        .with_system(gui::game_speed_widget)
        .with_system(gui::bug_live_info_system)
        .with_system(gui::bug_attribute_info_system)
        .with_system(gui::egg_live_info_panel_system)
        .with_system(gui::egg_attribute_info_panel_system)
        .with_system(gui::plant_info_panel_system)
        .with_system(gui::global_ui_update_system)
        .with_system(gui::bug_brain_info_system)
        .with_system(gui::bug_stats_info_system)
}

pub fn selection_system_set() -> SystemSet {
    ConditionSet::new()
        .run_if_not(gui::using_gui)
        .with_system(gui::select_sprite_system)
        .into()
}

pub fn global_statistics_system_set() -> SystemSet {
    ConditionSet::new()
        .run_if_not(interaction::is_paused)
        .with_system(statistics::count_system)
        .with_system(statistics::energy_stats_system)
        .with_system(statistics::performance_stats_system)
        .with_system(statistics::attribute_stats_system)
        .into()
}

pub struct GenesisUiPlugin;

impl Plugin for GenesisUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_fixed_timestep(Duration::from_millis(100), "stats")
            .add_fixed_timestep(Duration::from_secs(60), "regular_saver")
            .add_fixed_timestep_system("regular_saver", 0, regular_saver)
            .add_fixed_timestep_system_set("stats", 0, global_statistics_system_set())
            .add_system_set(selection_system_set())
            .add_system_set(interaction_system_set())
            .insert_resource(AverageAttributeStatistics::default())
            .insert_resource(CountStatistics::default())
            .insert_resource(BugPerformanceStatistics::default())
            .insert_resource(EnergyStatistics::default())
            .insert_resource(EntityPanelState::default())
            .insert_resource(GlobalPanelState::default())
            .insert_resource(interaction::SimulationSpeed::default())
            .add_system_to_stage(CoreStage::Last, save_on_close);
    }
}
