use crate::game::{BuildStates, GameStates};
use crate::grid::save_load::GridItem::{Cable, EnemyPC, Router, Switch, PC};
use crate::grid::{Grid, GRID_M, GRID_N};
use crate::shop::shop_items::ItemType;
use bevy::asset::io::Writer;
use bevy::math::uvec2;
use bevy::prelude::*;
use bevy::reflect::erased_serde::__private::serde;
use bevy::utils::info;
use bevy_common_assets::json::JsonAssetPlugin;
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(serde::Serialize, serde::Deserialize, Asset, TypePath, Default)]
pub struct GridState {
    items: Vec<GridItem>,
}

#[derive(serde::Serialize, serde::Deserialize, Asset, TypePath)]
pub enum GridItem {
    PC(UVec2),
    EnemyPC(UVec2),
    Router(UVec2),
    Switch(UVec2),
    Cable(URect),
}

impl GridItem {
    fn from_type(
        entity: Entity,
        item_type: &ItemType,
        grid: &Grid,
        (i, j): (usize, usize),
    ) -> Self {
        match item_type {
            ItemType::PC => PC(uvec2(i as u32, j as u32)),
            ItemType::EnemyPC => EnemyPC(uvec2(i as u32, j as u32)),
            ItemType::Router => Router(uvec2(i as u32, j as u32)),
            ItemType::Switch => Switch(uvec2(i as u32, j as u32)),
            ItemType::Cable => Cable(grid.cable_rect(entity)),
        }
    }
}

pub struct SaveLoadPlugin;

impl Plugin for SaveLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(JsonAssetPlugin::<GridState>::new(&["grid.json"]));
        app.add_systems(
            Update,
            save.run_if(in_state(BuildStates::Internal))
                .run_if(in_state(GameStates::InGame)),
        );
    }
}

fn save(grid: Res<Grid>, keys: Res<ButtonInput<KeyCode>>, items: Query<&ItemType>) {
    if keys.pressed(KeyCode::ControlLeft) {
        if keys.just_pressed(KeyCode::KeyS) {
            let mut state = GridState::default();

            for i in 0..GRID_N {
                for j in 0..GRID_M {
                    let Some(entity) = grid.grid[i][j] else {
                        continue;
                    };

                    let item_type = items.get(entity).unwrap();
                    state
                        .items
                        .push(GridItem::from_type(entity, &item_type, &grid, (i, j)));
                }
            }

            info!("writing to file");

            let mut writer = BufWriter::new(File::create("assets/grids/test.grid.json").unwrap());
            serde_json::to_writer(&mut writer, &state).unwrap();
            writer.flush().unwrap();
        }
    }
}
