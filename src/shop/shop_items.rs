use bevy::prelude::*;

use crate::{
    game::InGame,
    items::{
        cables::{Cable, CableDirection},
        enemy_pc::EnemyPC,
        pcs::PC,
        routers::Router,
        servers::Server,
        switches::Switch,
    },
};

#[derive(Component, Clone, Copy)]
pub enum ItemType {
    PC,
    EnemyPC,
    Router,
    Switch,
    Cable(CableDirection),
    Server,
}

impl ItemType {
    pub fn sprite_path(&self) -> String {
        match self {
            ItemType::PC => "pc.png",
            ItemType::EnemyPC => "enemy_pc.png",
            ItemType::Router => "router.png",
            ItemType::Switch => "switch.png",
            ItemType::Cable(_) => "cable.png",
            ItemType::Server => "server.png",
        }
        .to_string()
    }
    pub(crate) fn price(&self) -> u32 {
        match self {
            ItemType::Router => 20,
            ItemType::Switch => 25,
            ItemType::Cable(_) => 5,
            ItemType::Server => 30,

            _ => 0,
        }
    }
    pub fn name(&self) -> String {
        match self {
            ItemType::PC => "PC",
            ItemType::EnemyPC => "Enemy PC",
            ItemType::Router => "Router",
            ItemType::Switch => "Switch",
            ItemType::Cable(_) => "Cable",
            ItemType::Server => "Server",
        }
        .to_string()
    }

    pub fn add_component(&self, entity_commands: &mut EntityCommands) {
        match self {
            Self::PC => entity_commands.insert(PC),
            Self::EnemyPC => entity_commands.insert(EnemyPC),
            Self::Router => entity_commands.insert(Router),
            Self::Switch => entity_commands.insert(Switch),
            Self::Cable(dir) => entity_commands.insert(Cable { dir: *dir }),
            Self::Server => entity_commands.insert(Server),
        };
    }
}

#[derive(Component)]
#[require(InGame)]
pub struct ShopUI;

#[derive(Component)]
#[require(InGame)]
pub struct ShopRefID(pub Entity);

#[derive(Component)]
#[require(InGame)]
pub struct ShopPosition(pub(crate) Vec2);

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
            ShopPosition(pos),
            item_type,
            ShopRefID(ui_id),
            Sprite::from_image(asset_server.load(item_type.sprite_path())),
            Transform::from_translation(pos.extend(0.)),
            Name::new(item_type.name()),
        ))
        .id()
}
