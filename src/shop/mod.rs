pub mod currency;
use currency::*;

use bevy::{math::vec3, prelude::*};

use crate::{
    camera::init_camera,
    items::{Cable, PC, Router, Switch},
};

pub struct ShopPlugin;

#[derive(Component)]
pub struct ShopItem {
    pub item_type: ItemType,
    pub pos: Vec2,
    pub price: u32,
}
impl ShopItem {
    pub fn new(item_type: ItemType, pos: Vec2, price: u32) -> Self {
        Self {
            item_type,
            pos,
            price,
        }
    }
}

pub enum ItemType {
    PC,
    Router,
    Switch,
    Cable,
}

#[derive(Component)]
pub struct CableFromShop;

#[derive(Component)]
pub struct ShopUI;

#[derive(Component)]
pub struct ShopRefID(Entity);

impl ItemType {
    pub fn add_component(&self, entity_commands: &mut EntityCommands) {
        match self {
            Self::PC => entity_commands.insert(PC),
            Self::Router => entity_commands.insert(Router),
            Self::Switch => entity_commands.insert(Switch),
            Self::Cable => entity_commands.insert(Cable),
        };
    }
}

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

    let pos = vec3(camera_x - 50., -30., 0.);
    let router_id = commands
        .spawn((
            ShopUI,
            Text::new("Router, $15"),
            TextFont {
                font_size: 14.0,
                ..Default::default()
            },
        ))
        .id();

    commands.spawn((
        ShopRefID(router_id),
        ShopItem::new(ItemType::Router, pos.truncate(), 15),
        Sprite::from_image(asset_server.load("router.png")),
        Transform::from_translation(pos),
        Name::new("Router"),
    ));

    let switch_id = commands
        .spawn((
            ShopUI,
            Text::new("Switch, $10"),
            TextFont {
                font_size: 14.0,
                ..Default::default()
            },
        ))
        .id();
    let pos = vec3(camera_x - 25., -30., 0.);
    commands.spawn((
        ShopRefID(switch_id),
        ShopItem::new(ItemType::Switch, pos.truncate(), 10),
        Sprite::from_image(asset_server.load("switch.png")),
        Transform::from_translation(pos),
        Name::new("Switch"),
    ));

    let pos = vec3(camera_x + 0., -30., 0.);
    commands.spawn((
        ShopItem::new(ItemType::PC, pos.truncate(), 100),
        Sprite::from_image(asset_server.load("pc.png")),
        Transform::from_translation(pos),
        Name::new("PC"),
    ));

    let cable_id = commands
        .spawn((
            ShopUI,
            Text::new("Cable, $1"),
            TextFont {
                font_size: 14.0,
                ..Default::default()
            },
        ))
        .id();
    let pos = vec3(camera_x + 25., -30., 0.);
    commands.spawn((
        CableFromShop,
        ShopRefID(cable_id),
        ShopItem::new(ItemType::Cable, pos.truncate(), 1),
        Sprite::from_image(asset_server.load("cable.png")),
        Transform::from_translation(pos),
        Name::new("Cable"),
    ));
}

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

        node.top = Val::Px(pos.y + 21. / 2.);
        node.left = Val::Px(pos.x - c_node.size().x / 2.);
    }
}
