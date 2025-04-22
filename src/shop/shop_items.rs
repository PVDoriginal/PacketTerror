use bevy::{math::vec3, prelude::*};

use crate::camera::SCALE;

#[derive(Component)]
pub struct ShopItem {
    item_type: ItemType,
    pos: Vec2,
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
    fn add_component(&self, entity_commands: &mut EntityCommands) {
        match self {
            Self::PC => entity_commands.insert(PC),
            Self::Router => entity_commands.insert(Router),
            Self::Switch => entity_commands.insert(Switch),
        };
    }
}

#[derive(Component)]
struct PC;
#[derive(Component)]
struct Router;

#[derive(Component)]
struct Switch;

pub fn init_shop_items(mut commands: Commands, asset_server: Res<AssetServer>) {
    let pos = vec3(0., -50., 0.);
    commands
        .spawn((
            ShopItem::new(ItemType::Router, pos.truncate()),
            Sprite::from_image(asset_server.load("router.png")),
            Transform::from_translation(pos),
        ))
        .observe(drag_item)
        .observe(drop_item);

    let pos = vec3(-30., -50., 0.);
    commands
        .spawn((
            ShopItem::new(ItemType::Switch, pos.truncate()),
            Sprite::from_image(asset_server.load("switch.png")),
            Transform::from_translation(pos),
        ))
        .observe(drag_item)
        .observe(drop_item);
    let pos = vec3(30., -50., 0.);
    commands
        .spawn((
            ShopItem::new(ItemType::PC, pos.truncate()),
            Sprite::from_image(asset_server.load("pc.png")),
            Transform::from_translation(pos),
            Name::new("PC"),
        ))
        .observe(drag_item)
        .observe(drop_item);
}

pub fn drag_item(
    trigger: Trigger<Pointer<Drag>>,
    mut transforms: Query<&mut Transform, With<ShopItem>>,
) {
    let Ok(mut transform) = transforms.get_mut(trigger.entity()) else {
        return;
    };
    let drag = trigger.event();
    transform.translation.z = 1.;
    transform.translation += vec3(drag.delta.x * SCALE, drag.delta.y * -SCALE, 0.0);
}

pub fn drop_item(
    trigger: Trigger<Pointer<DragEnd>>,
    mut transforms: Query<(&mut Transform, &ShopItem, &Sprite)>,
    mut commands: Commands,
) {
    let Ok((mut transform, shop_item, sprite)) = transforms.get_mut(trigger.entity()) else {
        return;
    };
    transform.translation.z = 0.;

    let mut obj = commands.spawn((
        sprite.clone(),
        Transform::from_translation(transform.translation),
    ));
    shop_item.item_type.add_component(&mut obj);

    transform.translation = shop_item.pos.extend(0.);
}
