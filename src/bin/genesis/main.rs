use bevy::{
    prelude::{default, App, PluginGroup},
    window::{WindowDescriptor, WindowPlugin},
    DefaultPlugins,
};
use bevy_egui::EguiPlugin;
use bevy_rapier2d::prelude::*;
use genesis::GenesisPlugin;

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
        .run();
}
