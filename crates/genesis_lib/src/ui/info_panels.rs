use bevy::prelude::{Local, Query, Res, ResMut, Resource, With};
use bevy_egui::{egui, EguiContext};
use bevy_trait_query::ReadTraits;
use components::{body, eat, grab, grow, lay, see, time, Size};
use genesis_components as components;
use genesis_ecosystem as ecosystem;
use genesis_traits::AttributeDisplay;

use crate::{
    statistics,
    ui::{brain_panel, interaction::Selected},
};

#[derive(Debug, PartialEq, Eq, Default)]
pub enum GlobalPanel {
    #[default]
    Environment,
    Performance,
}

fn global_panel_buttons(ui: &mut egui::Ui, global_panel_state: &mut GlobalPanel) {
    ui.horizontal(|ui| {
        ui.selectable_value(global_panel_state, GlobalPanel::Environment, "Environment");
        ui.selectable_value(global_panel_state, GlobalPanel::Performance, "Performance");
    });
    ui.end_row();
}

pub fn global_ui_update_system(
    time: Res<time::SimulationTime>,
    count_stats: Res<statistics::CountStats>,
    energy_stats: Res<statistics::EnergyStats>,
    performance_stats: Res<statistics::BugPerformance>,
    mut egui_ctx: ResMut<EguiContext>,
    mut panel_state: Local<GlobalPanel>,
) {
    egui::Window::new("Global Info")
        .anchor(egui::Align2::RIGHT_BOTTOM, [-5.0, -5.0])
        .show(egui_ctx.ctx_mut(), |ui| {
            global_panel_buttons(ui, &mut panel_state);
            match *panel_state {
                GlobalPanel::Environment => {
                    environment_sub_panel(ui, &time, &energy_stats, &count_stats);
                }
                GlobalPanel::Performance => population_sub_panel(ui, &performance_stats),
            };
        });
}

fn environment_sub_panel(
    ui: &mut egui::Ui,
    time: &Res<time::SimulationTime>,
    energy_stats: &Res<statistics::EnergyStats>,
    count_stats: &Res<statistics::CountStats>,
) {
    ui.label(format!(
        "Global energy: {}",
        energy_stats.current_available_energy()
    ));
    ui.label(format!("Time elapsed: {:.2}", **time));
    ui.label(format!(
        "Number of adults: {}",
        count_stats.current_adults()
    ));
    ui.label(format!(
        "Number of juveniles: {}",
        count_stats.current_juveniles()
    ));
    ui.label(format!("Number of eggs: {}", count_stats.current_eggs()));
    ui.label(format!(
        "Total food energy: {}",
        energy_stats.current_food_energy()
    ));
}

fn population_sub_panel(ui: &mut egui::Ui, performance_stats: &statistics::BugPerformance) {
    ui.label(format!(
        "Highest energy consumed: {}",
        performance_stats.current_highest_energy_consumed()
    ));
    ui.label(format!(
        "Most eggs laid: {}",
        performance_stats.current_most_eggs_laid()
    ));
    ui.label(format!(
        "Max generation: {}",
        performance_stats.current_max_generation()
    ));
    ui.label(format!(
        "Oldest bug age: {:.2}",
        performance_stats.current_oldest_bug()
    ));
}

#[derive(Debug, Default, Resource)]
pub struct EntityPanelState {
    pub bug_info_panel_state: BugInfoPanel,
    pub egg_info_panel_state: EggInfoPanel,
}

fn top_left_info_window(title: impl Into<egui::WidgetText>) -> egui::Window<'static> {
    egui::Window::new(title).anchor(egui::Align2::LEFT_TOP, [5.0, 5.0])
}

#[derive(Debug, PartialEq, Eq, Default)]
pub enum BugInfoPanel {
    #[default]
    Live,
    Attributes,
    Brain,
    Stats,
    EnergyFlow,
}

fn bug_panel_buttons(ui: &mut egui::Ui, bug_info_panel_state: &mut BugInfoPanel) {
    ui.horizontal(|ui| {
        ui.selectable_value(bug_info_panel_state, BugInfoPanel::Live, "Live");
        ui.selectable_value(bug_info_panel_state, BugInfoPanel::Attributes, "Attributes");
        ui.selectable_value(bug_info_panel_state, BugInfoPanel::Brain, "Brain");
        ui.selectable_value(bug_info_panel_state, BugInfoPanel::Stats, "Statistics");
        ui.selectable_value(
            bug_info_panel_state,
            BugInfoPanel::EnergyFlow,
            "Energy Flow",
        );
    });
    ui.end_row();
}

type BugLiveInfo<'a> = (
    &'a time::Age,
    &'a body::Vitality,
    &'a Size,
    &'a see::Vision,
    &'a time::InternalTimer,
    &'a components::Generation,
    &'a components::SizeMultiplier,
);

pub fn bug_live_info_system(
    bug_query: Query<BugLiveInfo, With<Selected>>,
    mut egui_ctx: ResMut<EguiContext>,
    mut panel_state: ResMut<EntityPanelState>,
) {
    let Ok(bug_info) = bug_query.get_single() else {
        return;
    };
    if panel_state.bug_info_panel_state == BugInfoPanel::Live {
        top_left_info_window("Bug Live Info").show(egui_ctx.ctx_mut(), |ui| {
            bug_panel_buttons(ui, &mut panel_state.bug_info_panel_state);
            bug_live_sub_panel(ui, &bug_info);
        });
    }
}

fn bug_live_sub_panel(ui: &mut egui::Ui, bug_info: &BugLiveInfo) {
    ui.label(format!("Age: {:.2}", &bug_info.0.elapsed_secs()));
    ui.label(format!("Energy: {}", &bug_info.1.energy_store()));
    ui.label(format!("Health: {}", &bug_info.1.health()));
    ui.label(format!("Size: {:.2}", **bug_info.2));
    ui.label(format!("Visible Bugs: {}", &bug_info.3.visible_bugs()));
    ui.label(format!("Visible Food: {}", &bug_info.3.visible_food()));
    ui.label(format!("Internal timer: {:.2}", &bug_info.4.elapsed_secs()));
    ui.label(format!("Generation: {}", &bug_info.5 .0));
    ui.label(format!("Size Multiplier: {:.2}", &bug_info.6.as_float()));
}

pub fn attribute_info_system(
    is_egg_query: Query<&components::Egg, With<Selected>>,
    attr_query_part: Query<&dyn AttributeDisplay, With<Selected>>,
    mut egui_ctx: ResMut<EguiContext>,
    mut panel_state: ResMut<EntityPanelState>,
) {
    let Ok(attr_info_part) = attr_query_part.get_single() else {
        return;
    };
    if is_egg_query.get_single().is_err() {
        if panel_state.bug_info_panel_state == BugInfoPanel::Attributes {
            top_left_info_window("Bug Attribute Info").show(egui_ctx.ctx_mut(), |ui| {
                bug_panel_buttons(ui, &mut panel_state.bug_info_panel_state);
                attribute_sub_panel(ui, &attr_info_part);
            });
        }
    } else if panel_state.egg_info_panel_state == EggInfoPanel::Attributes {
        top_left_info_window("Egg Attribute Info").show(egui_ctx.ctx_mut(), |ui| {
            egg_panel_buttons(ui, &mut panel_state.egg_info_panel_state);
            attribute_sub_panel(ui, &attr_info_part);
        });
    }
}

fn attribute_sub_panel(ui: &mut egui::Ui, bug_info_part: &ReadTraits<dyn AttributeDisplay>) {
    for attr in bug_info_part {
        ui.label(attr.display());
    }
}

pub fn bug_brain_info_system(
    brain_query: Query<brain_panel::BugBrainInfo, With<Selected>>,
    mut egui_ctx: ResMut<EguiContext>,
    mut panel_state: ResMut<EntityPanelState>,
) {
    let Ok(bug_info) = brain_query.get_single() else {
        return;
    };
    if panel_state.bug_info_panel_state == BugInfoPanel::Brain {
        top_left_info_window("Bug Brain Info").show(egui_ctx.ctx_mut(), |ui| {
            bug_panel_buttons(ui, &mut panel_state.bug_info_panel_state);
            brain_panel::bug_brain_sub_panel(ui, &bug_info);
        });
    }
}

type BugStatsInfo<'a> = (&'a eat::EnergyConsumed, &'a lay::EggsLaid);

pub fn bug_stats_info_system(
    bug_query: Query<BugStatsInfo, With<Selected>>,
    mut egui_ctx: ResMut<EguiContext>,
    mut panel_state: ResMut<EntityPanelState>,
) {
    let Ok(bug_info) = bug_query.get_single() else {
        return;
    };
    if panel_state.bug_info_panel_state == BugInfoPanel::Stats {
        top_left_info_window("Bug Statistics").show(egui_ctx.ctx_mut(), |ui| {
            bug_panel_buttons(ui, &mut panel_state.bug_info_panel_state);
            bug_stat_sub_panel(ui, &bug_info);
        });
    }
}

fn bug_stat_sub_panel(ui: &mut egui::Ui, bug_stats: &BugStatsInfo) {
    ui.label(format!("Energy consumed: {}", **bug_stats.0));
    ui.label(format!("Eggs laid: {}", **bug_stats.1));
}

type EnergyFlowInfo<'a> = (
    &'a eat::EatingSum,
    &'a grab::GrabbingSum,
    &'a grow::GrowingSum,
    &'a grow::SizeSum,
    &'a lay::LayingSum,
    &'a components::TranslationSum,
    &'a components::RotationSum,
    &'a components::ThinkingSum,
);

pub fn energy_flow_info_system(
    bug_query: Query<EnergyFlowInfo, With<Selected>>,
    mut egui_ctx: ResMut<EguiContext>,
    mut panel_state: ResMut<EntityPanelState>,
) {
    let Ok(bug_info) = bug_query.get_single() else {
        return;
    };
    if panel_state.bug_info_panel_state == BugInfoPanel::EnergyFlow {
        top_left_info_window("Bug Statistics").show(egui_ctx.ctx_mut(), |ui| {
            bug_panel_buttons(ui, &mut panel_state.bug_info_panel_state);
            energy_flow_sub_panel(ui, &bug_info);
        });
    }
}

const MULTIPLIER: f32 = 20.0;

fn energy_flow_sub_panel(ui: &mut egui::Ui, energy_flow_info: &EnergyFlowInfo) {
    let total = (energy_flow_info.0.rate()
        + energy_flow_info.1.rate()
        + energy_flow_info.2.rate()
        + energy_flow_info.3.rate()
        + energy_flow_info.4.rate()
        + energy_flow_info.5.rate()
        + energy_flow_info.6.rate()
        + energy_flow_info.7.rate())
        * MULTIPLIER;
    ui.label(format!(
        "Eating: {:.2}",
        energy_flow_info.0.rate() * MULTIPLIER
    ));
    ui.label(format!(
        "Grabbing: {:.2}",
        energy_flow_info.1.rate() * MULTIPLIER
    ));
    ui.label(format!(
        "Growing: {:.2}",
        energy_flow_info.2.rate() * MULTIPLIER
    ));
    ui.label(format!(
        "Metabolism: {:.2}",
        energy_flow_info.3.rate() * MULTIPLIER
    ));
    ui.label(format!(
        "Laying: {:.2}",
        energy_flow_info.4.rate() * MULTIPLIER
    ));
    ui.label(format!(
        "Moving: {:.2}",
        energy_flow_info.5.rate() * MULTIPLIER
    ));
    ui.label(format!(
        "Rotating: {:.2}",
        energy_flow_info.6.rate() * MULTIPLIER
    ));
    ui.label(format!(
        "Thinking: {:.2}",
        energy_flow_info.7.rate() * MULTIPLIER
    ));
    ui.label(format!("Total: {total:.2}"));
}

#[derive(Debug, PartialEq, Eq, Default)]
pub enum EggInfoPanel {
    #[default]
    Live,
    Attributes,
}

fn egg_panel_buttons(ui: &mut egui::Ui, egg_info_panel_state: &mut EggInfoPanel) {
    ui.horizontal(|ui| {
        ui.selectable_value(egg_info_panel_state, EggInfoPanel::Live, "Live");
        ui.selectable_value(egg_info_panel_state, EggInfoPanel::Attributes, "Attributes");
    });
    ui.end_row();
}

type EggLiveInfo<'a> = (&'a time::Age, &'a components::Generation);

pub fn egg_live_info_panel_system(
    egg_query: Query<EggLiveInfo, (With<Selected>, With<ecosystem::EggEnergy>)>,
    mut egui_ctx: ResMut<EguiContext>,
    mut panel_state: ResMut<EntityPanelState>,
) {
    let Ok(egg_info) = egg_query.get_single() else {
        return;
    };
    if panel_state.egg_info_panel_state == EggInfoPanel::Live {
        top_left_info_window("Egg Live Info").show(egui_ctx.ctx_mut(), |ui| {
            egg_panel_buttons(ui, &mut panel_state.egg_info_panel_state);
            egg_live_sub_panel(ui, &egg_info);
        });
    }
}

fn egg_live_sub_panel(ui: &mut egui::Ui, egg_info: &EggLiveInfo) {
    ui.label(format!("Age: {:.2}", &egg_info.0.elapsed_secs()));
    ui.label(format!("Generation: {}", &egg_info.1 .0));
}

type FoodInfo<'a> = &'a ecosystem::Food;

pub fn food_info_panel_system(
    food_query: Query<FoodInfo, With<Selected>>,
    mut egui_ctx: ResMut<EguiContext>,
) {
    let Ok(food_info) = food_query.get_single() else {
        return;
    };
    top_left_info_window("Food Info").show(egui_ctx.ctx_mut(), |ui| {
        ui.label(format!("Energy: {}", &food_info.energy()));
    });
}
