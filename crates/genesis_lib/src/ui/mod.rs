mod brain_panel;
mod gui;
mod interaction;
pub mod menus;

use std::{fs, time::Duration};

use bevy::{
    app::AppExit,
    prelude::{App, CoreStage, EventReader, Plugin, Res, SystemSet},
    time::Time,
};
use genesis_config::WorldConfig;
pub use gui::{EntityPanelState, GlobalPanelState, LoadBugEvent, Selected};
pub use interaction::is_paused;
use iyes_loopless::prelude::*;
use serde_derive::Serialize;

use self::interaction::SimulationSpeed;
use crate::{statistics, SimState};

#[derive(Debug, Serialize)]
struct RunInfo<'a> {
    time_elapsed: &'a f32,
    run_config: &'a WorldConfig,
    count_stats: &'a statistics::CountStats,
    energy_stats: &'a statistics::EnergyStats,
    performance_stats: &'a statistics::BugPerformance,
    family_tree: &'a statistics::FamilyTree,
}

impl<'a> RunInfo<'a> {
    const fn new(
        time_elapsed: &'a f32,
        run_config: &'a WorldConfig,
        count_stats: &'a statistics::CountStats,
        energy_stats: &'a statistics::EnergyStats,
        performance_stats: &'a statistics::BugPerformance,
        family_tree: &'a statistics::FamilyTree,
    ) -> Self {
        Self {
            time_elapsed,
            run_config,
            count_stats,
            energy_stats,
            performance_stats,
            family_tree,
        }
    }
}

fn save_stats(
    time: &Res<Time>,
    count_stats: &Res<statistics::CountStats>,
    energy_stats: &Res<statistics::EnergyStats>,
    performance_stats: &Res<statistics::BugPerformance>,
    family_tree: &Res<statistics::FamilyTree>,
) {
    let time = time.elapsed_seconds();
    let run_info = RunInfo::new(
        &time,
        WorldConfig::global(),
        count_stats,
        energy_stats,
        performance_stats,
        family_tree,
    );
    let j = serde_json::to_string_pretty(&run_info).unwrap();
    fs::write("./run_data.json", j).expect("Unable to write file.");
}

pub fn save_on_close(
    events: EventReader<AppExit>,
    time: Res<Time>,
    count_stats: Res<statistics::CountStats>,
    energy_stats: Res<statistics::EnergyStats>,
    performance_stats: Res<statistics::BugPerformance>,
    family_tree: Res<statistics::FamilyTree>,
) {
    if !events.is_empty() {
        save_stats(
            &time,
            &count_stats,
            &energy_stats,
            &performance_stats,
            &family_tree,
        );
    }
}

pub fn regular_saver(
    time: Res<Time>,
    count_stats: Res<statistics::CountStats>,
    energy_stats: Res<statistics::EnergyStats>,
    performance_stats: Res<statistics::BugPerformance>,
    family_tree: Res<statistics::FamilyTree>,
) {
    save_stats(
        &time,
        &count_stats,
        &energy_stats,
        &performance_stats,
        &family_tree,
    );
}

pub fn interaction_system_set() -> SystemSet {
    ConditionSet::new()
        .run_in_state(SimState::Simulation)
        .with_system(interaction::move_camera_system)
        .with_system(interaction::camera_zooming_system)
        .with_system(interaction::pause_key_system)
        .with_system(interaction::pause_system)
        .with_system(gui::game_speed_widget)
        .with_system(gui::bug_live_info_system)
        .with_system(gui::attribute_info_system)
        .with_system(gui::egg_live_info_panel_system)
        .with_system(gui::plant_info_panel_system)
        .with_system(gui::global_ui_update_system)
        .with_system(gui::bug_brain_info_system)
        .with_system(gui::bug_stats_info_system)
        .with_system(gui::bug_serde_widget)
        .with_system(gui::bug_spawner_widget)
        .into()
}

fn simulation_speed_changed(speed: Res<SimulationSpeed>) -> bool {
    speed.is_changed()
}

pub fn game_time_system_set() -> SystemSet {
    ConditionSet::new()
        .run_in_state(SimState::Simulation)
        .run_if(simulation_speed_changed)
        .with_system(interaction::game_time_system)
        .into()
}

pub fn selection_system_set() -> SystemSet {
    ConditionSet::new()
        .run_if_not(gui::using_ui)
        .run_in_state(SimState::Simulation)
        .with_system(gui::select_sprite_system)
        .into()
}

pub fn manual_spawn_system_set() -> SystemSet {
    ConditionSet::new()
        .run_if_not(gui::using_ui)
        .run_in_state(SimState::Simulation)
        .with_system(gui::spawn_at_mouse)
        .into()
}

pub fn global_statistics_system_set() -> SystemSet {
    ConditionSet::new()
        .run_in_state(SimState::Simulation)
        .run_if_not(interaction::is_paused)
        .with_system(statistics::count_system)
        .with_system(statistics::energy_stats_system)
        .with_system(statistics::performance_stats_system)
        .into()
}

pub fn save_on_close_set() -> SystemSet {
    ConditionSet::new()
        .run_in_state(SimState::Simulation)
        .run_if_not(interaction::is_paused)
        .with_system(save_on_close)
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
            .add_system_set(manual_spawn_system_set())
            .add_system_set(game_time_system_set())
            .insert_resource(EntityPanelState::default())
            .insert_resource(GlobalPanelState::default())
            .insert_resource(interaction::SimulationSpeed::default())
            .add_event::<gui::SaveSimulationEvent>()
            .add_event::<gui::LoadBugEvent>()
            .add_system_set_to_stage(CoreStage::Last, save_on_close_set());
    }
}
