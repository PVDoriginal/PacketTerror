use bevy::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::CameraPlugin;
use game::GamePlugin;
use grid::GridPlugin;
use health::HealthPlugin;
use items::ItemsPlugin;
use main_menu::MainMenuPlugin;
use shop::ShopPlugin;

pub mod camera;
pub mod game;
pub mod grid;
pub mod health;
pub mod items;
pub mod main_menu;
pub mod shop;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins((
            CameraPlugin,
            GamePlugin,
            GridPlugin,
            ShopPlugin,
            ItemsPlugin,
            MainMenuPlugin,
            HealthPlugin,
        ))
        .run();
}
