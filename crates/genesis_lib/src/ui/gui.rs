use std::fs;

use bevy::{
    log::warn,
    prelude::{
        Camera, Color, Commands, Component, Entity, GlobalTransform, Input, MouseButton, Query,
        Res, ResMut, Resource, With,
    },
    sprite::Sprite,
    window::Windows,
};
use bevy_egui::{egui, EguiContext};
use bevy_rapier2d::prelude::{QueryFilter, RapierContext};
use genesis_attributes as attributes;
use genesis_components as components;
use genesis_components::{body, eat, lay, mind, see, time};
use genesis_ecosystem as ecosystem;
use serde::{Deserialize, Serialize};

use super::{brain_panel, interaction, statistics};
#[derive(Debug, Default, Resource)]
pub struct GlobalPanelState(pub GlobalPanel);

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
    mut panel_state: ResMut<GlobalPanelState>,
) {
    egui::Window::new("Global Info")
        .anchor(egui::Align2::RIGHT_BOTTOM, [-5.0, -5.0])
        .show(egui_ctx.ctx_mut(), |ui| {
            global_panel_buttons(ui, &mut panel_state.0);
            match panel_state.0 {
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

pub fn using_ui(mut egui_context: ResMut<EguiContext>) -> bool {
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

#[derive(Debug, PartialEq, Eq, Default)]
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
    &'a time::Age,
    &'a body::Vitality,
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
    ui.label(format!("Size Multiplier: {:.2}", &bug_info.5.as_float()));
}

pub fn attribute_info_system(
    is_egg_query: Query<&components::Egg, With<Selected>>,
    attr_query_part: Query<attributes::BugAttributes, With<Selected>>,
    mut egui_ctx: ResMut<EguiContext>,
    mut panel_state: ResMut<EntityPanelState>,
) {
    if let Ok(attr_info_part) = attr_query_part.get_single() {
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
}

fn attribute_sub_panel(ui: &mut egui::Ui, bug_info_part: &attributes::BugAttributes) {
    ui.label(format!("Hatch age: {:.3}", **bug_info_part.0));
    ui.label(format!("Adult Age: {:.3}", **bug_info_part.1));
    ui.label(format!("Death Age: {:.3}", **bug_info_part.2));
    ui.label(format!("Eye range: {:.3}", **bug_info_part.3));
    ui.label(format!(
        "Eye angle: {:.3}",
        f32::to_degrees(**bug_info_part.4)
    ));
    ui.label(format!("Cost of eating: {:.3}", **bug_info_part.5));
    ui.label(format!("Offspring energy: {:.3}", **bug_info_part.6));
    ui.label(format!(
        "Mouth width: {:.3}",
        f32::to_degrees(**bug_info_part.7)
    ));
    ui.label(format!("Hatch size: {:.3}", **bug_info_part.8));
    ui.label(format!("Maximum size: {:.3}", **bug_info_part.9));
    ui.label(format!("Growth rate: {:.3}", **bug_info_part.10));
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

type BugStatsInfo<'a> = (&'a eat::EnergyConsumed, &'a lay::EggsLaid);

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

#[derive(Debug, Serialize, Deserialize)]
pub struct BugBlueprint {
    mind: mind::Mind,
    genome: attributes::Genome,
}

pub fn bug_serde_widget(
    mut egui_ctx: ResMut<EguiContext>,
    bug_query: Query<(&mind::Mind, &attributes::Genome), With<Selected>>,
) {
    let Ok(bug) = bug_query.get_single() else {
        return;
    };
    egui::Window::new("Save")
        .anchor(egui::Align2::LEFT_BOTTOM, [5.0, -5.0])
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                if ui.button("Save bug").clicked() {
                    let path = std::env::current_dir().unwrap();
                    let Some(res) = rfd::FileDialog::new()
                        .set_file_name("bug.json")
                        .set_directory(path)
                        .save_file() else
                    {
                        return;
                    };
                    let bug_info = BugBlueprint {
                        mind: bug.0.to_owned(),
                        genome: bug.1.to_owned(),
                    };
                    let bug_json = serde_json::to_string_pretty(&bug_info).unwrap();
                    if let Err(e) = fs::write(res, bug_json) {
                        warn!("Could not save bug. Please try again. {e}")
                    };
                }
            })
        });
}
