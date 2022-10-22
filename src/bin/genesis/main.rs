use bevy::{prelude::App, window::WindowDescriptor, DefaultPlugins};
use bevy_egui::EguiPlugin;
use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};
use genesis::GenesisPlugin;

fn main() {
    App::new()
        .add_plugin(GenesisPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1000.0))
        .insert_resource(WindowDescriptor {
            title: "Genesis".to_string(),
            ..Default::default()
        })
        .run();
}
