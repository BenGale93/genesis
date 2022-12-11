use bevy_egui::egui;
use genesis_brain::{GuiNeuron, Synapses};
use genesis_util::color;
use itertools::Itertools;

use crate::mind;

pub(super) type BugBrainInfo<'a> = (&'a mind::MindInput, &'a mind::Mind, &'a mind::MindOutput);

const RADIUS: f32 = 20.0;
const SPACING: f32 = 20.0;
const START_POS: (f32, f32) = (30.0, 100.0);
const COLOR_ARRAY: &[(u8, u8, u8)] = &[(255, 0, 0), (160, 160, 160), (0, 150, 0)];
const NEURON_NAMES: [&str; 20] = [
    "Constant",
    "Movement",
    "Rotation",
    "Energy",
    "Health",
    "Age",
    "Visible bugs",
    "Bug angle",
    "Bug distance",
    "Visible food",
    "Food angle",
    "Food distance",
    "Heartbeat",
    "Internal timer",
    "Movement",
    "Rotation",
    "Reproduce",
    "Eat",
    "Reset timer",
    "Want to grow",
];

fn paint_synapses(ui: &mut egui::Ui, synapses: &Synapses, neuron_layout: &[GuiNeuron]) {
    let sorted_synapses = synapses
        .iter()
        .sorted_by(|a, b| a.active().cmp(&b.active()));
    for syn in sorted_synapses {
        let start_pos = &neuron_layout[syn.from()];
        let end_pos = &neuron_layout[syn.to()];

        let (Some((start_x, start_y)), Some((end_x, end_y))) = (start_pos.pos, end_pos.pos) else {
            continue;
        };

        let color = if syn.active() {
            let (r, g, b) = color::interpolate_color(syn.weight(), COLOR_ARRAY);
            egui::Color32::from_rgb(r, g, b)
        } else {
            egui::Color32::BLACK
        };

        ui.painter().line_segment(
            [egui::pos2(start_x, start_y), egui::pos2(end_x, end_y)],
            egui::Stroke::new(5.0, color),
        );
    }
}

fn paint_neuron_values(
    ui: &mut egui::Ui,
    neuron_index: usize,
    neuron_position: egui::Pos2,
    mind_values: &[f32],
) {
    let pos_val = mind_values.get(neuron_index);
    if let Some(val) = pos_val {
        ui.painter().text(
            neuron_position,
            egui::Align2::CENTER_CENTER,
            format!("{val:.2}"),
            egui::FontId::default(),
            egui::Color32::WHITE,
        );
    }
}

fn paint_neuron_labels(
    ui: &mut egui::Ui,
    response: &egui::Response,
    neuron_index: usize,
    neuron_position: egui::Pos2,
) {
    if let Some(hover_pos) = response.hover_pos() {
        let dist = (neuron_position - hover_pos).length();
        if dist < RADIUS {
            let label = NEURON_NAMES.get(neuron_index).map_or("", |text| *text);
            ui.painter().text(
                egui::pos2(380.0, 42.0),
                egui::Align2::LEFT_TOP,
                label,
                egui::FontId::proportional(16.0),
                egui::Color32::WHITE,
            );
        }
    }
}

fn paint_neurons(
    ui: &mut egui::Ui,
    response: &egui::Response,
    neuron_layout: &[GuiNeuron],
    mind_values: &[f32],
) {
    for gui_neuron in neuron_layout {
        let Some((x, y)) = gui_neuron.pos else {
            continue;
        };
        let neuron_position = egui::Pos2::new(x, y);

        let (r, g, b) = color::interpolate_color(gui_neuron.bias, COLOR_ARRAY);
        let color = egui::Color32::from_rgb(r, g, b);

        ui.painter().circle_filled(neuron_position, RADIUS, color);

        paint_neuron_values(ui, gui_neuron.index, neuron_position, mind_values);
        paint_neuron_labels(ui, response, gui_neuron.index, neuron_position);
    }
}

pub(super) fn bug_brain_sub_panel(ui: &mut egui::Ui, brain_info: &BugBrainInfo) {
    let (mind_in, mind, mind_out) = brain_info;

    let mut mind_values: Vec<f32> = mind_in.iter().copied().collect();
    mind_values.extend(&mind_out.0);

    let neuron_layout = mind.layout_neurons(&START_POS, RADIUS, SPACING);

    let (_rect, response) =
        ui.allocate_exact_size(egui::Vec2::new(1000.0, 680.0), egui::Sense::hover());

    paint_synapses(ui, mind.synapses(), &neuron_layout);
    paint_neurons(ui, &response, &neuron_layout, &mind_values);
}
