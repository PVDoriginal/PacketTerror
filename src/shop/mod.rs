pub mod currency;
pub mod shop_items;
use currency::*;

use bevy::{math::vec2, prelude::*};
use shop_items::{ItemType, ShopItem, ShopRefID, ShopUI, spawn_shop_item};

use crate::camera::{init_camera, SPRITE_SIZE};

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CurrencyPlugin);
        app.add_systems(Startup, init_shop_items.after(init_camera));
        app.add_systems(Update, move_shop_ui);
        app.add_event::<UpdateCurrencyEvent>();
    }
}

pub fn init_shop_items(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera: Query<&Transform, With<Camera2d>>,
) {
    let Ok(camera_x) = camera.get_single().map(|x| x.translation.x) else {
        return;
    };

    let shop_items = vec![
        ItemType::Router,
        ItemType::Switch,
        ItemType::PC,
        ItemType::Cable,
    ];

    const ITEM_SPACE: usize = 50;
    let n = shop_items.len();

    for (index, item) in shop_items.iter().enumerate() {
        let x: f32 = camera_x + index as f32 * ITEM_SPACE as f32 - (ITEM_SPACE * n) as f32 / 2.;
        spawn_shop_item(&mut commands, &asset_server, *item, vec2(x, -30.));
    }
}

// maybe move into a generalized UI module?
fn move_shop_ui(
    positions: Query<(&Transform, &ShopRefID), With<ShopItem>>,
    cameras: Query<(&GlobalTransform, &Camera)>,
    mut shop_ui: Query<(&mut Node, &ComputedNode), With<ShopUI>>,
) {
    let (camera_transform, camera) = cameras.single();
    for (position, ui_id) in &positions {
        let pos = camera
            .world_to_viewport(camera_transform, position.translation)
            .expect("camera panik");

        let Ok((mut node, c_node)) = shop_ui.get_mut(ui_id.0) else {
            continue;
        };

        node.top = Val::Px(pos.y + SPRITE_SIZE / 2.);
        node.left = Val::Px(pos.x - c_node.size().x / 2.);
    }
}
