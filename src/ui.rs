use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::{body, ecosystem, interaction, mind, sight::Vision};

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

#[derive(Component)]
pub struct Selected;

pub fn select_bug_system(
    mut commands: Commands,
    wnds: Res<Windows>,
    mouse_button: Res<Input<MouseButton>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut bug_query: Query<(Entity, &Transform, &mut Sprite), With<mind::Mind>>,
) {
    if !mouse_button.pressed(MouseButton::Left) {
        return;
    }
    // check if the cursor is inside the window and get its position
    if let Some(world_pos) = interaction::get_cursor_position(wnds, q_camera) {
        for (entity, transform, mut sprite) in bug_query.iter_mut() {
            let dist = (world_pos - transform.translation.truncate()).length();
            if dist < 9.0 {
                commands.entity(entity).insert(Selected);
                sprite.color = Color::RED;
            } else {
                commands.entity(entity).remove::<Selected>();
                sprite.color = Color::WHITE;
            }
        }
    }
}

type BugInfo<'a> = (
    &'a Transform,
    &'a body::Age,
    &'a body::Vitality,
    &'a Velocity,
    &'a Vision,
    &'a body::InternalTimer,
);

fn populate_bug_info(bug_info: &BugInfo, mut info_text: Query<&mut Text, With<BugInfoText>>) {
    let mut text = info_text.single_mut();
    text.sections[1].value = format!("\nPosition: {}", &bug_info.0.translation.truncate());
    text.sections[2].value = format!("\nRotation: {}", &bug_info.0.rotation.z);
    text.sections[3].value = format!("\nAge: {}", &bug_info.1);
    text.sections[4].value = format!("\nEnergy: {}", &bug_info.2.energy_store());
    text.sections[5].value = format!("\nHealth: {}", &bug_info.2.health());
    text.sections[6].value = format!("\nVelocity: {}", &bug_info.3.linvel);
    text.sections[7].value = format!("\nVisible Bugs: {}", &bug_info.4.visible_bugs());
    text.sections[8].value = format!("\nVisible Food: {}", &bug_info.4.visible_food());
    text.sections[9].value = format!("\nInternal Timer: {}", &bug_info.5);
}

fn spawn_info_panel(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let text_style = TextStyle {
        font: asset_server.load("fonts/calibri.ttf"),
        font_size: 30.0,
        color: Color::WHITE,
    };
    commands
        .spawn_bundle(
            // Create a TextBundle that has a Text with a list of sections.
            TextBundle::from_sections([
                TextSection::new("Bug Info", text_style.clone()),
                TextSection::from_style(text_style.clone()),
                TextSection::from_style(text_style.clone()),
                TextSection::from_style(text_style.clone()),
                TextSection::from_style(text_style.clone()),
                TextSection::from_style(text_style.clone()),
                TextSection::from_style(text_style.clone()),
                TextSection::from_style(text_style.clone()),
                TextSection::from_style(text_style.clone()),
                TextSection::from_style(text_style),
            ])
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            }),
        )
        .insert(BugInfoText);
}

#[derive(Component)]
pub struct BugInfoText;

pub fn selected_bug_system(
    mut commands: Commands,
    bug_query: Query<BugInfo, With<Selected>>,
    info_panel_query: Query<Entity, With<BugInfoText>>,
    info_text: Query<&mut Text, With<BugInfoText>>,
    asset_server: Res<AssetServer>,
) {
    match (bug_query.get_single(), info_panel_query.get_single()) {
        (Ok(_), Err(_)) => spawn_info_panel(&mut commands, asset_server),
        (Err(_), Ok(info_panel)) => commands.entity(info_panel).despawn(),
        (Ok(bug_info), Ok(_)) => populate_bug_info(&bug_info, info_text),
        _ => (),
    }
}
