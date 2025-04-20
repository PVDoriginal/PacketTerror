use bevy::prelude::*;

use bevy_inspector_egui::bevy_egui::EguiPlugin;
use camera::CameraPlugin;
use game::GamePlugin;

pub mod camera;
pub mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(EguiPlugin)
        .add_plugins((GamePlugin, CameraPlugin))
        .run();
}
