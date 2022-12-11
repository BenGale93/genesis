use bevy::{
    prelude::{
        Camera, Color, Commands, Component, Entity, GlobalTransform, Input, MouseButton, Query,
        Res, ResMut, Resource, With,
    },
    sprite::Sprite,
    window::Windows,
};
use bevy_egui::{egui, EguiContext};
use bevy_rapier2d::prelude::{QueryFilter, RapierContext};

use super::{brain_panel, interaction, statistics};
use crate::{
    attributes,
    behaviour::{
        eating, lifecycle, sight,
        timers::{self, SimulationTime},
    },
    body, ecosystem,
};

#[derive(Debug, Default, Resource)]
pub struct GlobalPanelState(pub GlobalPanel);

#[derive(Debug, PartialEq, Default)]
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
    time: Res<SimulationTime>,
    count_stats: Res<statistics::CountStatistics>,
    energy_stats: Res<statistics::EnergyStatistics>,
    performance_stats: Res<statistics::BugPerformanceStatistics>,
    mut egui_ctx: ResMut<EguiContext>,
    mut panel_state: ResMut<GlobalPanelState>,
) {
    egui::Window::new("Global Info")
        .anchor(egui::Align2::RIGHT_BOTTOM, [-5.0, -5.0])
        .show(egui_ctx.ctx_mut(), |ui| {
            global_panel_buttons(ui, &mut panel_state.0);
            match panel_state.0 {
                GlobalPanel::Environment => {
                    environment_sub_panel(ui, &time, &energy_stats, &count_stats)
                }
                GlobalPanel::Performance => population_sub_panel(ui, &performance_stats),
            };
        });
}

fn environment_sub_panel(
    ui: &mut egui::Ui,
    time: &Res<SimulationTime>,
    energy_stats: &Res<statistics::EnergyStatistics>,
    count_stats: &Res<statistics::CountStatistics>,
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

fn population_sub_panel(
    ui: &mut egui::Ui,
    performance_stats: &statistics::BugPerformanceStatistics,
) {
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

pub fn using_gui(mut egui_context: ResMut<EguiContext>) -> bool {
    let ctx = egui_context.ctx_mut();
    ctx.is_using_pointer() || ctx.is_pointer_over_area()
}

#[derive(Component)]
pub struct Selected;

pub fn select_sprite_system(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    wnds: Res<Windows>,
    mouse_button: Res<Input<MouseButton>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut sprite_query: Query<(Entity, &mut Sprite, &body::OriginalColor)>,
) {
    let filter = QueryFilter::default();
    if !mouse_button.pressed(MouseButton::Left) {
        return;
    }
    // check if the cursor is inside the window and get its position
    if let Some(world_pos) = interaction::get_cursor_position(wnds, q_camera) {
        for (entity, mut sprite, original_color) in sprite_query.iter_mut() {
            commands.entity(entity).remove::<Selected>();
            sprite.color = original_color.0;
        }
        rapier_context.intersections_with_point(world_pos, filter, |selected_entity| {
            for (entity, mut sprite, _) in sprite_query.iter_mut() {
                if selected_entity == entity {
                    commands.entity(selected_entity).insert(Selected);
                    sprite.color = Color::RED;
                }
            }
            false
        });
    }
}

#[derive(Debug, Default, Resource)]
pub struct EntityPanelState {
    pub bug_info_panel_state: BugInfoPanel,
    pub egg_info_panel_state: EggInfoPanel,
}

fn top_left_info_window(title: impl Into<egui::WidgetText>) -> egui::Window<'static> {
    egui::Window::new(title).anchor(egui::Align2::LEFT_TOP, [5.0, 5.0])
}

#[derive(Debug, PartialEq, Default)]
pub enum BugInfoPanel {
    #[default]
    Live,
    Attributes,
    Brain,
    Stats,
}

fn bug_panel_buttons(ui: &mut egui::Ui, bug_info_panel_state: &mut BugInfoPanel) {
    ui.horizontal(|ui| {
        ui.selectable_value(bug_info_panel_state, BugInfoPanel::Live, "Live");
        ui.selectable_value(bug_info_panel_state, BugInfoPanel::Attributes, "Attributes");
        ui.selectable_value(bug_info_panel_state, BugInfoPanel::Brain, "Brain");
        ui.selectable_value(bug_info_panel_state, BugInfoPanel::Stats, "Statistics");
    });
    ui.end_row();
}

type BugLiveInfo<'a> = (
    &'a timers::Age,
    &'a body::Vitality,
    &'a sight::Vision,
    &'a timers::InternalTimer,
    &'a lifecycle::Generation,
);

pub fn bug_live_info_system(
    bug_query: Query<BugLiveInfo, With<Selected>>,
    mut egui_ctx: ResMut<EguiContext>,
    mut panel_state: ResMut<EntityPanelState>,
) {
    if let Ok(bug_info) = bug_query.get_single() {
        if panel_state.bug_info_panel_state == BugInfoPanel::Live {
            top_left_info_window("Bug Live Info").show(egui_ctx.ctx_mut(), |ui| {
                bug_panel_buttons(ui, &mut panel_state.bug_info_panel_state);
                bug_live_sub_panel(ui, &bug_info);
            });
        }
    }
}

fn bug_live_sub_panel(ui: &mut egui::Ui, bug_info: &BugLiveInfo) {
    ui.label(format!("Age: {:.2}", &bug_info.0.elapsed_secs()));
    ui.label(format!("Energy: {}", &bug_info.1.energy_store()));
    ui.label(format!("Health: {}", &bug_info.1.health()));
    ui.label(format!("Size: {}", &bug_info.1.size().current_size()));
    ui.label(format!("Visible Bugs: {}", &bug_info.2.visible_bugs()));
    ui.label(format!("Visible Food: {}", &bug_info.2.visible_food()));
    ui.label(format!("Internal timer: {:.2}", &bug_info.3.elapsed_secs()));
    ui.label(format!("Generation: {}", &bug_info.4 .0));
}

pub fn bug_attribute_info_system(
    bug_query_part1: Query<attributes::BugAttributesPart1, With<Selected>>,
    bug_query_part2: Query<attributes::BugAttributesPart2, With<Selected>>,
    mut egui_ctx: ResMut<EguiContext>,
    mut panel_state: ResMut<EntityPanelState>,
) {
    if let (Ok(bug_info_part1), Ok(bug_info_part2)) =
        (bug_query_part1.get_single(), bug_query_part2.get_single())
    {
        if panel_state.bug_info_panel_state == BugInfoPanel::Attributes {
            top_left_info_window("Bug Attribute Info").show(egui_ctx.ctx_mut(), |ui| {
                bug_panel_buttons(ui, &mut panel_state.bug_info_panel_state);
                bug_attribute_sub_panel(ui, &bug_info_part1, &bug_info_part2);
            });
        }
    }
}

fn bug_attribute_sub_panel(
    ui: &mut egui::Ui,
    bug_info_part1: &attributes::BugAttributesPart1,
    bug_info_part2: &attributes::BugAttributesPart2,
) {
    ui.label(format!("Adult Age: {}", **bug_info_part1.0));
    ui.label(format!("Death Age: {}", **bug_info_part1.1));
    ui.label(format!("Eye angle: {:.3}", **bug_info_part1.2));
    ui.label(format!("Eye range: {}", **bug_info_part1.3));
    ui.label(format!("Max rotation: {}", &bug_info_part1.4.value()));
    ui.label(format!("Rotation cost: {:.3}", &bug_info_part1.4.cost()));
    ui.label(format!("Max speed: {}", &bug_info_part1.5.value()));
    ui.label(format!("Movement cost: {:.3}", &bug_info_part1.5.cost()));
    ui.label(format!(
        "Mutation Probability: {:.3}",
        &bug_info_part1.6.as_float()
    ));
    ui.label(format!("Offspring energy: {}", **bug_info_part1.7));
    ui.label(format!("Lay egg boundary: {:.3}", **bug_info_part1.8));
    ui.label(format!(
        "Internal timer boundary: {:.3}",
        **bug_info_part1.9
    ));
    ui.label(format!("Growing boundary: {:.3}", **bug_info_part1.10));
    ui.label(format!("Eating boundary: {:.3}", **bug_info_part1.11));
    ui.label(format!("Cost of thought: {:.3}", **bug_info_part1.12));
    ui.label(format!("Cost of eating: {:.3}", **bug_info_part1.13));
    ui.label(format!("Hatch size: {:.3}", **bug_info_part1.14));
    ui.label(format!("Maximum size: {:.3}", **bug_info_part1.0));
    ui.label(format!("Growth rate: {:.3}", **bug_info_part2.1));
    ui.label(format!("Mouth width: {:.3}", **bug_info_part2.2));
}

pub fn bug_brain_info_system(
    brain_query: Query<brain_panel::BugBrainInfo, With<Selected>>,
    mut egui_ctx: ResMut<EguiContext>,
    mut panel_state: ResMut<EntityPanelState>,
) {
    if let Ok(bug_info) = brain_query.get_single() {
        if panel_state.bug_info_panel_state == BugInfoPanel::Brain {
            top_left_info_window("Bug Brain Info").show(egui_ctx.ctx_mut(), |ui| {
                bug_panel_buttons(ui, &mut panel_state.bug_info_panel_state);
                brain_panel::bug_brain_sub_panel(ui, &bug_info);
            });
        }
    }
}

type BugStatsInfo<'a> = (&'a eating::EnergyConsumed, &'a lifecycle::EggsLaid);

pub fn bug_stats_info_system(
    bug_query: Query<BugStatsInfo, With<Selected>>,
    mut egui_ctx: ResMut<EguiContext>,
    mut panel_state: ResMut<EntityPanelState>,
) {
    if let Ok(bug_info) = bug_query.get_single() {
        if panel_state.bug_info_panel_state == BugInfoPanel::Stats {
            top_left_info_window("Bug Statistics").show(egui_ctx.ctx_mut(), |ui| {
                bug_panel_buttons(ui, &mut panel_state.bug_info_panel_state);
                bug_stat_sub_panel(ui, &bug_info);
            });
        }
    }
}

fn bug_stat_sub_panel(ui: &mut egui::Ui, bug_stats: &BugStatsInfo) {
    ui.label(format!("Energy consumed: {}", **bug_stats.0));
    ui.label(format!("Eggs laid: {}", **bug_stats.1));
}

#[derive(Debug, PartialEq, Default)]
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

type EggLiveInfo<'a> = (&'a timers::Age, &'a lifecycle::Generation);

pub fn egg_live_info_panel_system(
    egg_query: Query<EggLiveInfo, (With<Selected>, With<lifecycle::EggEnergy>)>,
    mut egui_ctx: ResMut<EguiContext>,
    mut panel_state: ResMut<EntityPanelState>,
) {
    if let Ok(egg_info) = egg_query.get_single() {
        if panel_state.egg_info_panel_state == EggInfoPanel::Live {
            top_left_info_window("Egg Live Info").show(egui_ctx.ctx_mut(), |ui| {
                egg_panel_buttons(ui, &mut panel_state.egg_info_panel_state);
                egg_live_sub_panel(ui, &egg_info);
            });
        }
    }
}

fn egg_live_sub_panel(ui: &mut egui::Ui, egg_info: &EggLiveInfo) {
    ui.label(format!("Age: {:.2}", &egg_info.0.elapsed_secs()));
    ui.label(format!("Generation: {}", &egg_info.1 .0));
}

type EggAttributeInfo<'a> = &'a attributes::HatchAge;

pub fn egg_attribute_info_panel_system(
    egg_query: Query<EggAttributeInfo, With<Selected>>,
    mut egui_ctx: ResMut<EguiContext>,
    mut panel_state: ResMut<EntityPanelState>,
) {
    if let Ok(egg_info) = egg_query.get_single() {
        if panel_state.egg_info_panel_state == EggInfoPanel::Attributes {
            top_left_info_window("Egg Attribute Info").show(egui_ctx.ctx_mut(), |ui| {
                egg_panel_buttons(ui, &mut panel_state.egg_info_panel_state);
                egg_attribute_sub_panel(ui, &egg_info);
            });
        }
    }
}

fn egg_attribute_sub_panel(ui: &mut egui::Ui, egg_info: &EggAttributeInfo) {
    ui.label(format!("Hatch age: {}", ***egg_info));
}

type PlantInfo<'a> = &'a ecosystem::Plant;

pub fn plant_info_panel_system(
    plant_query: Query<PlantInfo, With<Selected>>,
    mut egui_ctx: ResMut<EguiContext>,
) {
    if let Ok(plant_info) = plant_query.get_single() {
        top_left_info_window("Plant Info").show(egui_ctx.ctx_mut(), |ui| {
            ui.label(format!("Energy: {}", &plant_info.energy()));
        });
    }
}

pub fn game_speed_widget(
    mut egui_ctx: ResMut<EguiContext>,
    mut speed: ResMut<interaction::SimulationSpeed>,
) {
    let symbol = if speed.paused { "⏵" } else { "⏸" };
    egui::Window::new("Controls")
        .anchor(egui::Align2::RIGHT_TOP, [-5.0, 5.0])
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                if ui.button(symbol).clicked() {
                    speed.paused = !speed.paused;
                }
                ui.add(egui::Slider::new(&mut speed.speed, 0.1..=3.0).text("Game Speed"))
            })
        });
}
