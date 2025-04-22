use bevy::{math::vec3, prelude::*};

use crate::{camera::SCALE, shop::ShopItem};

use super::Grid;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, make_interactable);
    }
}

pub fn make_interactable(mut commands: Commands, shop_items: Query<Entity, Added<ShopItem>>) {
    for item in &shop_items {
        commands.entity(item).observe(drag_item).observe(drop_item);
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
) {
    let Ok((mut transform, shop_item, sprite)) = transforms.get_mut(trigger.entity()) else {
        return;
    };
    transform.translation.z = 0.;

    let Some(pos) = grid.world_to_grid(transform.translation.truncate()) else {
        transform.translation = shop_item.pos.extend(0.);
        return;
    };

    let mut obj = commands.spawn((
        sprite.clone(),
        Transform::from_translation(pos.extend(0).as_vec3() * 21.),
    ));
    grid.grid[pos.x as usize][pos.y as usize] = Some(obj.id());
    shop_item.item_type.add_component(&mut obj);

    transform.translation = shop_item.pos.extend(0.);
}
