use bevy::{
    ecs::schedule::ShouldRun,
    prelude::{
        Camera, Color, Commands, Component, Entity, GlobalTransform, Input, MouseButton, Query,
        Res, ResMut, With,
    },
    sprite::Sprite,
    window::Windows,
};
use bevy_egui::{egui, EguiContext};
use bevy_rapier2d::prelude::{QueryFilter, RapierContext};

use super::{brain_panel, interaction, statistics};
use crate::{
    attributes,
    behaviour::{lifecycle, sight, timers},
    body, ecosystem,
};

#[allow(clippy::too_many_arguments)]
pub fn global_ui_update_system(
    mut egui_ctx: ResMut<EguiContext>,
    global_stats: Res<statistics::GlobalStatistics>,
) {
    egui::Window::new("Global Info")
        .anchor(egui::Align2::RIGHT_BOTTOM, [-5.0, -5.0])
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.label(format!(
                "Global energy: {}",
                global_stats.energy_stats().current_available_energy()
            ));
            ui.label(format!("Time elapsed: {:.2}", global_stats.time_elapsed()));
            ui.label(format!(
                "Max generation: {}",
                global_stats.current_max_generation()
            ));
            ui.label(format!(
                "Number of adults: {}",
                global_stats.count_stats().current_adults()
            ));
            ui.label(format!(
                "Number of juveniles: {}",
                global_stats.count_stats().current_juveniles()
            ));
            ui.label(format!(
                "Number of eggs: {}",
                global_stats.count_stats().current_eggs()
            ));
            ui.label(format!(
                "Total food energy: {}",
                global_stats.energy_stats().current_food_energy()
            ));
        });
}

pub fn run_if_not_using_egui(mut egui_context: ResMut<EguiContext>) -> ShouldRun {
    let ctx = egui_context.ctx_mut();
    if ctx.is_using_pointer() || ctx.is_pointer_over_area() {
        ShouldRun::No
    } else {
        ShouldRun::Yes
    }
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

#[derive(Debug, Default)]
pub struct PanelState {
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
}

fn bug_panel_buttons(ui: &mut egui::Ui, bug_info_panel_state: &mut BugInfoPanel) {
    ui.horizontal(|ui| {
        ui.selectable_value(bug_info_panel_state, BugInfoPanel::Live, "Live");
        ui.selectable_value(bug_info_panel_state, BugInfoPanel::Attributes, "Attributes");
        ui.selectable_value(bug_info_panel_state, BugInfoPanel::Brain, "Brain");
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
    mut panel_state: ResMut<PanelState>,
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

type BugAttributeInfoPart1<'a> = (
    &'a attributes::AdultAge,
    &'a attributes::DeathAge,
    &'a attributes::EyeAngle,
    &'a attributes::EyeRange,
    &'a attributes::MaxRotationRate,
    &'a attributes::MaxSpeed,
    &'a attributes::MutationProbability,
    &'a attributes::OffspringEnergy,
    &'a attributes::LayEggBoundary,
    &'a attributes::InternalTimerBoundary,
    &'a attributes::WantToGrowBoundary,
    &'a attributes::EatingBoundary,
    &'a attributes::CostOfThought,
    &'a attributes::CostOfEating,
    &'a attributes::MaxSize,
);

type BugAttributeInfoPart2<'a> = (&'a attributes::GrowthRate,);

pub fn bug_attribute_info_system(
    bug_query_part1: Query<BugAttributeInfoPart1, With<Selected>>,
    bug_query_part2: Query<BugAttributeInfoPart2, With<Selected>>,
    mut egui_ctx: ResMut<EguiContext>,
    mut panel_state: ResMut<PanelState>,
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
    bug_info_part1: &BugAttributeInfoPart1,
    bug_info_part2: &BugAttributeInfoPart2,
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
    ui.label(format!("Maximum size: {:.3}", **bug_info_part1.14));
    ui.label(format!("Growth rate: {:.3}", **bug_info_part2.0));
}

pub fn bug_brain_info_system(
    brain_query: Query<brain_panel::BugBrainInfo, With<Selected>>,
    mut egui_ctx: ResMut<EguiContext>,
    mut panel_state: ResMut<PanelState>,
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
    mut panel_state: ResMut<PanelState>,
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
    mut panel_state: ResMut<PanelState>,
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
