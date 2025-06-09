use crate::camera::SPRITE_SIZE;
use crate::game::{BuildStates, GameLevels, GameStates};
use crate::grid::cable_interaction::{CableSpawnMode, spawn_cable};
use crate::grid::save_load::GridItem::{Cable, EnemyPC, PC, Router, Switch};
use crate::grid::{GRID_M, GRID_N, Grid};
use crate::items::cables::CableDirection;
use crate::shop::shop_items::ItemType;
use bevy::math::uvec2;
use bevy::prelude::*;
use bevy::reflect::erased_serde::__private::serde;
use bevy::utils::HashSet;
use bevy_common_assets::json::JsonAssetPlugin;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

#[derive(serde::Serialize, serde::Deserialize, Asset, TypePath, Default)]
pub struct GridState {
    items: Vec<GridItem>,
}

#[derive(Resource)]
pub struct GridHandle(Option<Handle<GridState>>);

#[derive(serde::Serialize, serde::Deserialize, Asset, TypePath, Debug)]
pub enum GridItem {
    PC(UVec2),
    EnemyPC(UVec2),
    Router(UVec2),
    Switch(UVec2),
    Cable(URect, CableDirection),
    Server(UVec2),
}

impl GridItem {
    fn from_type(entity: Entity, item_type: &ItemType, grid: &Grid, (i, j): (u32, u32)) -> Self {
        match item_type {
            ItemType::PC => PC(uvec2(i, j)),
            ItemType::EnemyPC => EnemyPC(uvec2(i, j)),
            ItemType::Router => Router(uvec2(i, j)),
            ItemType::Switch => Switch(uvec2(i, j)),
            ItemType::Cable(dir) => Cable(grid.cable_rect(entity, uvec2(i, j)), *dir),
            ItemType::Server => Self::Server(uvec2(i, j)),
        }
    }
}

impl Into<ItemType> for GridItem {
    fn into(self) -> ItemType {
        match self {
            GridItem::PC(_) => ItemType::PC,
            GridItem::EnemyPC(_) => ItemType::EnemyPC,
            GridItem::Router(_) => ItemType::Router,
            GridItem::Switch(_) => ItemType::Switch,
            GridItem::Cable(_, dir) => ItemType::Cable(dir),
            GridItem::Server(_) => ItemType::Server,
        }
    }
}

pub struct SaveLoadPlugin;

impl Plugin for SaveLoadPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GridHandle(None));

        app.add_plugins(JsonAssetPlugin::<GridState>::new(&["grid.json"]));

        app.add_systems(OnEnter(GameStates::InGame), load_on_play);
        app.add_systems(Update, populate_grid.run_if(in_state(GameStates::InGame)));

        app.add_systems(
            Update,
            save.run_if(in_state(BuildStates::Internal))
                .run_if(in_state(GameStates::InGame)),
        );
    }
}

// temporary
fn load_on_play(
    asset_server: Res<AssetServer>,
    level: Res<State<GameLevels>>,
    mut commands: Commands,
) {
    if !Path::new(&format!("assets/{}", level.level_path())).exists() {
        info!("path not exist: assets/{}", &level.level_path());
        return;
    }
    commands.insert_resource(GridHandle(Some(asset_server.load(level.level_path()))));
}

pub fn spawn_item(
    pos: UVec2,
    item_type: ItemType,
    grid: &mut Grid,
    asset_server: &AssetServer,
    commands: &mut Commands,
) -> Entity {
    let mut entity = commands.spawn((
        Name::new(item_type.name()),
        Sprite::from_image(asset_server.load(item_type.sprite_path())),
        item_type,
        Transform::from_translation((pos.as_vec2() * SPRITE_SIZE).extend(0.)),
    ));

    item_type.add_component(&mut entity);
    grid.grid[pos.x as usize][pos.y as usize] = Some(entity.id());

    entity.id()
}

fn populate_grid(
    mut commands: Commands,
    mut grid_handle: ResMut<GridHandle>,
    mut grids: ResMut<Assets<GridState>>,
    asset_server: Res<AssetServer>,
    mut grid: ResMut<Grid>,
) {
    let Some(handle) = grid_handle.0.as_ref().map(|a| a.id()) else {
        return;
    };

    let Some(grid_state) = grids.remove(handle) else {
        return;
    };

    grid.reset();

    grid_handle.0 = None;

    for grid_item in grid_state.items {
        match grid_item {
            GridItem::Cable(rect, dir) => {
                spawn_cable(
                    rect,
                    &mut commands,
                    &asset_server,
                    CableSpawnMode::Raw,
                    &mut grid,
                    dir,
                );
            }
            GridItem::PC(pos) => {
                spawn_item(
                    pos,
                    grid_item.into(),
                    &mut grid,
                    &asset_server,
                    &mut commands,
                );
            }
            GridItem::EnemyPC(pos) => {
                spawn_item(
                    pos,
                    grid_item.into(),
                    &mut grid,
                    &asset_server,
                    &mut commands,
                );
            }
            GridItem::Router(pos) => {
                spawn_item(
                    pos,
                    grid_item.into(),
                    &mut grid,
                    &asset_server,
                    &mut commands,
                );
            }
            GridItem::Switch(pos) => {
                spawn_item(
                    pos,
                    grid_item.into(),
                    &mut grid,
                    &asset_server,
                    &mut commands,
                );
            }
            GridItem::Server(pos) => {
                spawn_item(
                    pos,
                    grid_item.into(),
                    &mut grid,
                    &asset_server,
                    &mut commands,
                );
            }
        }
    }
}

fn save(
    grid: Res<Grid>,
    keys: Res<ButtonInput<KeyCode>>,
    items: Query<&ItemType>,
    level: Res<State<GameLevels>>,
) {
    if keys.pressed(KeyCode::ControlLeft) {
        if keys.just_pressed(KeyCode::KeyS) {
            let mut state = GridState::default();

            let mut added_entities: HashSet<Entity> = HashSet::new();

            for i in 0..GRID_N {
                for j in 0..GRID_M {
                    let Some(entity) = grid.grid[i][j] else {
                        continue;
                    };

                    if added_entities.contains(&entity) {
                        continue;
                    }

                    added_entities.insert(entity);

                    let item_type = items.get(entity).unwrap();
                    state.items.push(GridItem::from_type(
                        entity,
                        &item_type,
                        &grid,
                        (i as u32, j as u32),
                    ));
                }
            }

            info!("writing to file");

            let path = format!("assets/{}", level.level_path());

            let mut writer = BufWriter::new(File::create(path).unwrap());
            serde_json::to_writer(&mut writer, &state).unwrap();
            writer.flush().unwrap();
        }
    }
}
