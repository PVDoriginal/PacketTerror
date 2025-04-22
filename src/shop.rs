use bevy::{math::vec3, prelude::*};

use crate::items::{PC, Router, Switch};

pub struct ShopPlugin;

#[derive(Resource)]
pub struct Currency {
    value: i32,
}

#[derive(Component)]
pub struct ShopItem {
    pub item_type: ItemType,
    pub pos: Vec2,
}
impl ShopItem {
    pub fn new(item_type: ItemType, pos: Vec2) -> Self {
        Self { item_type, pos }
    }
}

pub enum ItemType {
    PC,
    Router,
    Switch,
}

impl ItemType {
    pub fn add_component(&self, entity_commands: &mut EntityCommands) {
        match self {
            Self::PC => entity_commands.insert(PC),
            Self::Router => entity_commands.insert(Router),
            Self::Switch => entity_commands.insert(Switch),
        };
    }
}

#[derive(Event)]
pub struct UpdateCurrencyEvent(i32);

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Currency { value: 0 });
        app.add_systems(Startup, init_shop_items);
        app.add_systems(Update, update_currency);
        app.add_event::<UpdateCurrencyEvent>();
    }
}

pub fn update_currency(
    mut currency: ResMut<Currency>,
    mut event_remove: EventReader<UpdateCurrencyEvent>,
) {
    for ev in event_remove.read() {
        currency.value += ev.0;
    }
}

pub fn init_shop_items(mut commands: Commands, asset_server: Res<AssetServer>) {
    let pos = vec3(0., -50., 0.);
    commands.spawn((
        ShopItem::new(ItemType::Router, pos.truncate()),
        Sprite::from_image(asset_server.load("router.png")),
        Transform::from_translation(pos),
    ));

    let pos = vec3(-30., -50., 0.);
    commands.spawn((
        ShopItem::new(ItemType::Switch, pos.truncate()),
        Sprite::from_image(asset_server.load("switch.png")),
        Transform::from_translation(pos),
    ));

    let pos = vec3(30., -50., 0.);
    commands.spawn((
        ShopItem::new(ItemType::PC, pos.truncate()),
        Sprite::from_image(asset_server.load("pc.png")),
        Transform::from_translation(pos),
        Name::new("PC"),
    ));
}
