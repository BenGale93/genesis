#![warn(clippy::all, clippy::nursery)]
#![feature(is_some_and)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::similar_names)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::prelude::{App, Plugin};
use iyes_loopless::prelude::*;
use simulation::SimulationPlugin;

mod behaviour;
mod genesis_serde;
mod lifecycle;
mod setup;
mod simulation;
mod spawning;
mod statistics;
mod ui;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SimState {
    MainMenu,
    Simulation,
    Saving,
    Loading,
}

pub struct GenesisPlugin;

impl Plugin for GenesisPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(SimState::MainMenu)
            .add_plugin(SimulationPlugin)
            .add_plugin(genesis_ecosystem::EcosystemPlugin)
            .add_plugin(ui::menus::MenusPlugin);
    }
}
