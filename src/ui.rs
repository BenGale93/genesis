use bevy::prelude::*;

use crate::ecosystem;

#[derive(Component)]
pub struct EnergyText;

pub fn energy_ui_update_system(
    ecosystem: Res<ecosystem::Ecosystem>,
    mut query: Query<&mut Text, With<EnergyText>>,
) {
    for mut text in &mut query {
        let energy = ecosystem.available_energy();
        text.sections[1].value = format!("{energy}");
    }
}
