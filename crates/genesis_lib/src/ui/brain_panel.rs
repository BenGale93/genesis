use bevy_egui::egui;
use genesis_components::mind;

pub(super) type BugBrainInfo<'a> = (
    &'a mind::MindInput,
    &'a mind::MindLayout,
    &'a mind::MindOutput,
);

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

fn paint_synapses(ui: &mut egui::Ui, synapses: &[mind::PaintedSynapse]) {
    for syn in synapses {
        ui.painter()
            .line_segment([syn.start, syn.end], egui::Stroke::new(5.0, syn.color));
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
        if dist < mind::RADIUS {
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
    neuron_layout: &[mind::GuiNeuron],
    mind_values: &[f32],
) {
    for gui_neuron in neuron_layout {
        let Some(neuron_position) = gui_neuron.pos else {
            continue;
        };

        ui.painter()
            .circle_filled(neuron_position, mind::RADIUS, gui_neuron.color);

        paint_neuron_values(ui, gui_neuron.index, neuron_position, mind_values);
        paint_neuron_labels(ui, response, gui_neuron.index, neuron_position);
    }
}

pub(super) fn bug_brain_sub_panel(ui: &mut egui::Ui, brain_info: &BugBrainInfo) {
    let (mind_in, mind_layout, mind_out) = brain_info;

    let mut mind_values: Vec<f32> = mind_in.iter().copied().collect();
    mind_values.extend(&mind_out.0);

    let (_rect, response) =
        ui.allocate_exact_size(egui::Vec2::new(1000.0, 680.0), egui::Sense::hover());

    paint_synapses(ui, mind_layout.synapses());
    paint_neurons(ui, &response, mind_layout.neurons(), &mind_values);
}