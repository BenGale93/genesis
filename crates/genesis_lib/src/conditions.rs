use bevy::prelude::{Res, ResMut};
use bevy_egui::EguiContext;

use crate::simulation::SimulationSpeed;

pub fn is_paused(speed: Res<SimulationSpeed>) -> bool {
    speed.paused
}

pub fn simulation_speed_changed(speed: Res<SimulationSpeed>) -> bool {
    speed.is_changed()
}

pub fn using_ui(mut egui_context: ResMut<EguiContext>) -> bool {
    let ctx = egui_context.ctx_mut();
    ctx.is_using_pointer() || ctx.is_pointer_over_area()
}
