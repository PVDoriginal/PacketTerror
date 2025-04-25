use bevy::{math::vec3, prelude::*};
use std::f32::consts::PI;

use super::{Grid, interaction::can_place_item};
use crate::items::EnemyPC;
use crate::shop::shop_items::ShopPosition;
use crate::{
    camera::SPRITE_SIZE,
    items::{Cable, PC, Router, Server, Switch},
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
    mut grid_vals: Query<
        Entity,
        Or<(
            With<PC>,
            With<Switch>,
            With<Router>,
            With<EnemyPC>,
            With<Server>,
        )>,
    >,
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
    grid_vals: &mut Query<
        Entity,
        Or<(
            With<PC>,
            With<Switch>,
            With<Router>,
            With<EnemyPC>,
            With<Server>,
        )>,
    >,
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
    mut grid_vals: Query<
        Entity,
        Or<(
            With<PC>,
            With<Switch>,
            With<Router>,
            With<EnemyPC>,
            With<Server>,
        )>,
    >,
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

    if pos1.x != pos2.x && pos1.y != pos2.y {
        return;
    }

    let price = pos1.as_vec2().distance(pos2.as_vec2()) as i32;

    if currency.value < price {
        return;
    }

    spawn_cable(
        URect::from_corners(pos1, pos2),
        &mut commands,
        &asset_server,
        CableSpawnMode::CutSides,
        Some(&mut grid),
    );

    writer.send(UpdateCurrencyEvent(-1 * price));
    info!("positions!{} -> {}", pos1, pos2);
}

#[derive(Eq, PartialEq)]
pub enum CableSpawnMode {
    CutSides,
    Raw,
}

pub fn spawn_cable(
    mut rect: URect,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mode: CableSpawnMode,
    grid: Option<&mut Grid>,
) -> Entity {
    let cable_parent = commands
        .spawn((
            Cable,
            ItemType::Cable,
            Name::new("Cable parent"),
            Transform::default(),
            Visibility::Visible,
        ))
        .id();

    let rotation = if rect.min.y == rect.max.y {
        Quat::IDENTITY
    } else {
        Quat::from_rotation_z(PI / 2.)
    };

    if mode == CableSpawnMode::CutSides {
        if rect.min.x == rect.max.x {
            rect.min.y += 1;
            rect.max.y -= 1;
        } else if rect.min.y == rect.max.y {
            rect.min.x += 1;
            rect.max.x -= 1;
        }
    }

    if let Some(grid) = grid {
        for x in rect.min.x..rect.max.x + 1 {
            for y in rect.min.y..rect.max.y + 1 {
                grid.grid[x as usize][y as usize] = Some(cable_parent);
            }
        }
    }

    for x in rect.min.x..rect.max.x + 1 {
        for y in rect.min.y..rect.max.y + 1 {
            commands
                .spawn((
                    Sprite::from_image(asset_server.load("cable.png")),
                    Transform::from_translation(vec3(
                        x as f32 * SPRITE_SIZE,
                        y as f32 * SPRITE_SIZE,
                        1.,
                    ))
                    .with_rotation(rotation),
                    Name::new("Cable child"),
                ))
                .set_parent(cable_parent);
        }
    }

    cable_parent
}
