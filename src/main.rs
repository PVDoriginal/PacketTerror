use bevy::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::CameraPlugin;
use game::GamePlugin;
use grid::Grid;
use shop::ShopPlugin;

pub mod camera;
pub mod game;
pub mod grid;
pub mod shop;

fn main() {
    App::new()
    .init_resource::<Grid>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins((GamePlugin, CameraPlugin, ShopPlugin))
        .run();
}
