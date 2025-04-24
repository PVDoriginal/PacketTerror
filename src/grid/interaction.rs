use bevy::{math::vec3, prelude::*};

use crate::{
    camera::{SCALE, SPRITE_SIZE},
    items::{PC, Router, Switch},
    shop::{
        currency::{Currency, UpdateCurrencyEvent},
        shop_items::{ItemType, ShopItem},
    },
};

use super::Grid;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, make_interactable);
    }
}

pub fn make_interactable(
    mut commands: Commands,
    shop_items: Query<(Entity, &ShopItem), Added<ShopItem>>,
) {
    for (item, item_type) in &shop_items {
        match item_type.item_type {
            ItemType::Cable => {
                commands.entity(item).observe(drag_item).observe(drop_cable);
            }
            _ => {
                commands.entity(item).observe(drag_item).observe(drop_item);
            }
        }
    }
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
    mut grid: ResMut<Grid>,
    currency: Res<Currency>,
    mut writer: EventWriter<UpdateCurrencyEvent>,
) {
    let Ok((mut transform, shop_item, sprite)) = transforms.get_mut(trigger.entity()) else {
        return;
    };
    transform.translation.z = 0.;
    if can_place_item(&transform, &shop_item, &grid, &currency)
        && grid.on_empty_cell(transform.translation.truncate())
    {
        let pos = grid
            .world_to_grid(transform.translation.truncate())
            .unwrap();

        let mut obj = commands.spawn((
            sprite.clone(),
            Transform::from_translation(pos.extend(0).as_vec3() * SPRITE_SIZE),
        ));
        shop_item.item_type.add_component(&mut obj);

        grid.grid[pos.x as usize][pos.y as usize] = Some(obj.id());
        writer.send(UpdateCurrencyEvent(-1 * shop_item.price as i32));
    }

    // snap back:
    transform.translation = shop_item.pos.extend(0.);
}

fn can_place_item(
    transform: &Mut<Transform>,
    shop_item: &ShopItem,
    grid: &ResMut<Grid>,
    currency: &Res<Currency>,
) -> bool {
    if currency.value < shop_item.price as i32 {
        return false;
    }
    grid.inside_grid(transform.translation.truncate())
}

pub fn drop_cable(
    trigger: Trigger<Pointer<DragEnd>>,
    mut transforms: Query<(&mut Transform, &ShopItem)>,
    mut grid_vals: Query<Entity, Or<(With<PC>, With<Switch>, With<Router>)>>,
    grid: ResMut<Grid>,
    currency: Res<Currency>,
) {
    let Ok((mut transform, shop_item)) = transforms.get_mut(trigger.entity()) else {
        return;
    };
    transform.translation.z = 0.;
    if can_place_item(&transform, &shop_item, &grid, &currency)
        && cable_can_connect(&transform, &grid, &mut grid_vals)
    {
        info!("yep!");
    }

    // snap back:
    transform.translation = shop_item.pos.extend(0.);
}

fn cable_can_connect(
    transform: &Mut<Transform>,
    grid: &ResMut<Grid>,
    grid_vals: &mut Query<Entity, Or<(With<PC>, With<Switch>, With<Router>)>>,
) -> bool {
    let Some(entity) = grid.get_element(transform.translation.truncate()) else {
        return false;
    };
    let Ok(_) = grid_vals.get_mut(entity) else {
        return false;
    };
    true
}
