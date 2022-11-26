use bevy::{
    prelude::{default, App, PluginGroup},
    window::{WindowDescriptor, WindowPlugin},
    DefaultPlugins,
};
use bevy_egui::EguiPlugin;
use bevy_rapier2d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use genesis::GenesisPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Genesis".to_string(),
                ..Default::default()
            },
            ..default()
        }))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(EguiPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1000.0))
        .add_plugin(GenesisPlugin)
        .run();
}
