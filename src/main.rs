use bevy::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::CameraPlugin;
use defeat_screen::DefeatScreenPlugin;
use game::GamePlugin;
use grid::GridPlugin;
use health::HealthPlugin;
use items::ItemsPlugin;
use levels_menu::LevelsPlugin;
use main_menu::MainMenuPlugin;
use shake::ShakePlugin;
use shop::ShopPlugin;
use victory_screen::VictoryScreenPlugin;

pub mod camera;
pub mod defeat_screen;
pub mod game;
pub mod grid;
pub mod health;
pub mod items;
pub mod levels;
pub mod levels_menu;
pub mod main_menu;
pub mod shake;
pub mod shop;
pub mod victory_screen;

#[cfg(test)]
pub mod testing;

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
            LevelsPlugin,
            MainMenuPlugin,
            HealthPlugin,
            ShakePlugin,
            DefeatScreenPlugin,
            VictoryScreenPlugin,
        ))
        .run();
}
