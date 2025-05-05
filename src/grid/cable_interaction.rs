use bevy::{math::vec3, prelude::*};
use std::f32::consts::PI;

use super::{Grid, interaction::can_place_item};
use crate::items::cables::{Cable, CableDirection};
use crate::items::enemy_pc::EnemyPC;
use crate::items::pc::PC;
use crate::items::routers::Router;
use crate::items::servers::Server;
use crate::items::switches::Switch;
use crate::shop::shop_items::ShopPosition;
use crate::{
    camera::SPRITE_SIZE,
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

    cable_state.set(CableState::Idle);

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

    let pos1 = grid.world_to_grid(cable.0).unwrap();
    let pos2 = grid.world_to_grid(pos).unwrap();

    if pos1.x != pos2.x && pos1.y != pos2.y {
        return;
    }

    let price = pos1.as_vec2().distance(pos2.as_vec2()) as i32;

    if currency.value < price {
        return;
    }

    let rect = URect::from_corners(pos1, pos2);

    let direction = if rect.min.y == rect.max.y {
        CableDirection::Horizontal
    } else {
        CableDirection::Vertical
    };

    spawn_cable(
        rect,
        &mut commands,
        &asset_server,
        CableSpawnMode::CutSides,
        &mut grid,
        direction,
    );

    writer.send(UpdateCurrencyEvent(-1 * price));
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
    grid: &mut Grid,
    dir: CableDirection,
) -> Option<Entity> {
    if mode == CableSpawnMode::CutSides {
        if rect.min.x == rect.max.x {
            rect.min.y += 1;
            rect.max.y -= 1;
        } else if rect.min.y == rect.max.y {
            rect.min.x += 1;
            rect.max.x -= 1;
        }
    }

    for x in rect.min.x..rect.max.x + 1 {
        for y in rect.min.y..rect.max.y + 1 {
            if grid.grid[x as usize][y as usize].is_some() {
                return None; // cannot place over existing entities
            }
        }
    }

    let cable_parent = commands
        .spawn((
            Cable { dir },
            ItemType::Cable(dir),
            Name::new("Cable parent"),
            Transform::default(),
            Visibility::Visible,
        ))
        .insert(Cable { dir })
        .id();

    let rotation = match dir {
        CableDirection::Horizontal => Quat::IDENTITY,
        CableDirection::Vertical => Quat::from_rotation_z(PI / 2.),
    };

    for x in rect.min.x..rect.max.x + 1 {
        for y in rect.min.y..rect.max.y + 1 {
            grid.grid[x as usize][y as usize] = Some(cable_parent);
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

    Some(cable_parent)
}
