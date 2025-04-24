use bevy::prelude::*;

use crate::items::{Cable, PC, Router, Switch};

#[derive(Clone, Copy)]
pub enum ItemType {
    PC,
    Router,
    Switch,
    Cable,
}

impl ItemType {
    fn sprite_path(&self) -> String {
        match self {
            ItemType::PC => "pc.png",
            ItemType::Router => "router.png",
            ItemType::Switch => "switch.png",
            ItemType::Cable => "cable.png",
        }
        .to_string()
    }
    fn price(&self) -> u32 {
        match self {
            ItemType::Router => 15,
            ItemType::Switch => 10,
            ItemType::Cable => 1,

            _ => 0,
        }
    }
    fn name(&self) -> String {
        match self {
            ItemType::PC => "PC",
            ItemType::Router => "Router",
            ItemType::Switch => "Switch",
            ItemType::Cable => "Cable",
        }
        .to_string()
    }

    pub fn add_component(&self, entity_commands: &mut EntityCommands) {
        match self {
            Self::PC => entity_commands.insert(PC),
            Self::Router => entity_commands.insert(Router),
            Self::Switch => entity_commands.insert(Switch),
            Self::Cable => entity_commands.insert(Cable),
        };
    }
}

#[derive(Component)]
pub struct ShopUI;

#[derive(Component)]
pub struct ShopRefID(pub Entity);

#[derive(Component)]
pub struct ShopItem {
    pub pos: Vec2,
    pub item_type: ItemType,
    pub price: u32,
}

pub fn spawn_shop_item(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    item_type: ItemType,
    pos: Vec2,
) -> Entity {
    let ui_id = commands
        .spawn((
            ShopUI,
            Text::new(format!("{}, ${}", item_type.name(), item_type.price())),
            TextFont {
                font_size: 14.0,
                ..Default::default()
            },
        ))
        .id();

    commands
        .spawn((
            ShopItem {
                pos,
                item_type,
                price: item_type.price(),
            },
            ShopRefID(ui_id),
            Sprite::from_image(asset_server.load(item_type.sprite_path())),
            Transform::from_translation(pos.extend(0.)),
            Name::new(item_type.name()),
        ))
        .id()
}
