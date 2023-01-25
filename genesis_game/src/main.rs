use bevy::{
    prelude::{default, App, PluginGroup},
    ui::{AlignSelf, PositionType, Style, UiRect, Val},
    window::{WindowDescriptor, WindowPlugin},
    DefaultPlugins,
};
use bevy_egui::EguiPlugin;
use bevy_rapier2d::prelude::*;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use genesis_lib::GenesisPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "Genesis".to_string(),
            ..Default::default()
        },
        ..default()
    }));

    #[cfg(debug_assertions)]
    app.add_plugin(RapierDebugRenderPlugin::default());

    app.add_plugin(EguiPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1000.0))
        .add_plugin(GenesisPlugin)
        .add_plugin(ScreenDiagnosticsPlugin {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.0),
                    left: Val::Percent(50.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .add_plugin(ScreenFrameDiagnosticsPlugin)
        .run();
}
