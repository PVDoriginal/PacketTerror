use bevy::prelude::*;
use bevy::window::{CursorOptions, WindowResolution};

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::CameraPlugin;
use game::GamePlugin;
use grid::GridPlugin;
use items::ItemsPlugin;
use shop::ShopPlugin;

pub mod camera;
pub mod game;
pub mod grid;
pub mod items;
pub mod main_menu;
pub mod shop;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;

fn window_setup() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Packet Terror".to_string(),
            resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            resizable: false,
            ..default()
        }),
        ..default()
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(window_setup())
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins((
            CameraPlugin,
            GamePlugin,
            GridPlugin,
            ShopPlugin,
            ItemsPlugin,
            //MainMenuPlugin,
        ))
        .run();
}
