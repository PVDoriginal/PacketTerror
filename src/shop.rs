use bevy::{math::vec3, prelude::*};

use crate::{
    camera::init_camera,
    items::{Cable, PC, Router, Switch},
};

#[derive(Resource)]
pub struct Currency {
    pub value: i32,
}
#[derive(Component)]
pub struct CurrencyDisplay;

#[derive(Event)]
pub struct UpdateCurrencyEvent(pub i32);

pub fn init_currency(mut commands: Commands, currency: Res<Currency>) {
    // Note: text without textBundle seems to float to screen top-left
    commands.spawn((
        CurrencyDisplay,
        Text::new(format!("Packet credits: {}", currency.value)),
        TextFont {
            font_size: 14.0,
            ..Default::default()
        },
    ));
}
pub struct CurrencyPlugin;
impl Plugin for CurrencyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Currency { value: 300 });
        app.add_systems(Startup, init_currency);
        app.add_systems(Update, update_currency);
    }
}
pub fn update_currency(
    mut currency: ResMut<Currency>,
    mut event_update: EventReader<UpdateCurrencyEvent>,
    mut display_currency: Query<&mut Text, With<CurrencyDisplay>>,
) {
    for ev in event_update.read() {
        if currency.value + ev.0 >= 0 {
            currency.value += ev.0;
        }
    }

    let Ok(mut text) = display_currency.get_single_mut() else {
        return;
    };
    text.0 = format!("Packet credits: {}", currency.value);
}

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
    commands.spawn((
        ShopItem::new(ItemType::Router, pos.truncate(), 15),
        Sprite::from_image(asset_server.load("router.png")),
        Transform::from_translation(pos),
        Name::new("Router"),
    ));

    let pos = vec3(camera_x - 25., -30., 0.);
    commands.spawn((
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

    let pos = vec3(camera_x + 25., -30., 0.);
    commands.spawn((
        CableFromShop,
        ShopItem::new(ItemType::Cable, pos.truncate(), 15),
        Sprite::from_image(asset_server.load("cable.png")),
        Transform::from_translation(pos),
        Name::new("Cable"),
    ));
}
