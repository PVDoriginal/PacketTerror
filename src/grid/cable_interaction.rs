use bevy::{math::vec3, prelude::*};
use std::f32::consts::PI;

use super::{interaction::can_place_item, Grid};
use crate::shop::shop_items::ShopPosition;
use crate::{
    camera::SPRITE_SIZE,
    items::{Cable, Router, Switch, PC},
    shop::{
        currency::{Currency, UpdateCurrencyEvent},
        shop_items::ItemType,
    },
};

#[derive(States, Debug, Default, Hash, Clone, Copy, Eq, PartialEq)]
pub enum CableState {
    #[default]
    Idle,
    Cabling,
}

#[derive(Resource, Default)]
pub struct CableOrigin(Vec2);

pub struct CableInteractionPlugin;

impl Plugin for CableInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CableOrigin>();
        app.init_state::<CableState>();
        app.add_systems(Update, click_cable.run_if(in_state(CableState::Cabling)));
    }
}

pub fn drop_cable(
    trigger: Trigger<Pointer<DragEnd>>,
    mut transforms: Query<(&mut Transform, &ShopPosition, &ItemType)>,
    mut grid_vals: Query<Entity, Or<(With<PC>, With<Switch>, With<Router>)>>,
    grid: ResMut<Grid>,
    mut cable: ResMut<CableOrigin>,
    currency: Res<Currency>,
    mut cable_state: ResMut<NextState<CableState>>,
) {
    let Ok((mut transform, shop_pos, item_type)) = transforms.get_mut(trigger.entity()) else {
        return;
    };
    transform.translation.z = 0.;
    if can_place_item(&transform, item_type, &grid, &currency)
        && cable_can_connect(&transform.translation.truncate(), &grid, &mut grid_vals)
    {
        cable_state.set(CableState::Cabling);
        cable.0 = transform.translation.truncate();
    }

    // snap back:
    transform.translation = shop_pos.0.extend(0.);
}

fn cable_can_connect(
    pos: &Vec2,
    grid: &ResMut<Grid>,
    grid_vals: &mut Query<Entity, Or<(With<PC>, With<Switch>, With<Router>)>>,
) -> bool {
    let Some(entity) = grid.get_element(*pos) else {
        return false;
    };
    let Ok(_) = grid_vals.get_mut(entity) else {
        return false;
    };
    true
}

pub fn click_cable(
    windows: Query<&Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut grid_vals: Query<Entity, Or<(With<PC>, With<Switch>, With<Router>)>>,
    mut grid: ResMut<Grid>,
    cameras: Query<(&GlobalTransform, &Camera)>,
    cable: Res<CableOrigin>,
    mut cable_state: ResMut<NextState<CableState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    currency: Res<Currency>,
    mut writer: EventWriter<UpdateCurrencyEvent>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    let Some(screen_pos) = windows.single().cursor_position() else {
        return;
    };

    let (camera_transform, camera) = cameras.single();
    let pos = camera
        .viewport_to_world(camera_transform, screen_pos)
        .expect("camera panik")
        .origin
        .truncate();

    if !grid.inside_grid(pos) {
        return;
    }
    if !cable_can_connect(&pos, &grid, &mut grid_vals) {
        return;
    }

    info!("can connect!{} -> {}", cable.0, pos);

    cable_state.set(CableState::Idle);

    let pos1 = grid.world_to_grid(cable.0).unwrap();
    let pos2 = grid.world_to_grid(pos).unwrap();

    info!("positions!{} -> {}", pos1, pos2);

    if pos1.x == pos2.x && pos1 != pos2 {
        let mn_pos = pos1.y.min(pos2.y) as usize;
        let mx_pos = pos1.y.max(pos2.y) as usize;

        let cable_cnt = (mx_pos - mn_pos - 1) as i32;
        if currency.value < cable_cnt {
            return;
        }

        let cable_parent = commands.spawn((Cable, Name::new("Cable parent"))).id();

        for j in mn_pos + 1..mx_pos {
            grid.grid[pos1.x as usize][j as usize] = Some(cable_parent);

            commands.spawn((
                Sprite::from_image(asset_server.load("cable.png")),
                Transform::from_translation(vec3(
                    pos1.x as f32 * SPRITE_SIZE,
                    j as f32 * SPRITE_SIZE,
                    1.,
                ))
                .with_rotation(Quat::from_rotation_z(PI / 2.)),
                Name::new("CableChild"),
            ));
        }
        writer.send(UpdateCurrencyEvent(-1 * cable_cnt));
    }

    if pos1.y == pos2.y && pos1 != pos2 {
        let mn_pos = pos1.x.min(pos2.x) as usize;
        let mx_pos = pos1.x.max(pos2.x) as usize;

        let cable_cnt = (mx_pos - mn_pos - 1) as i32;
        if currency.value < cable_cnt {
            return;
        }

        let cable_parent = commands
            .spawn((
                Cable,
                Name::new("Cable parent"),
                Transform::default(),
                Visibility::Visible,
            ))
            .id();
        for i in mn_pos + 1..mx_pos {
            grid.grid[i][pos1.y as usize] = Some(cable_parent);

            commands
                .spawn((
                    Sprite::from_image(asset_server.load("cable.png")),
                    Transform::from_translation(vec3(
                        i as f32 * SPRITE_SIZE,
                        pos1.y as f32 * SPRITE_SIZE,
                        1.,
                    )),
                    Name::new("Cable child"),
                ))
                .set_parent(cable_parent);
        }
        writer.send(UpdateCurrencyEvent(-1 * cable_cnt));
    }
}
