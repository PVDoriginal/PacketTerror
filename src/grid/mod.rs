use bevy::{
    math::{uvec2, vec3},
    prelude::*,
};
use interaction::InteractionPlugin;

use crate::camera::SPRITE_SIZE;

pub mod interaction;

pub const GRID_N: usize = 30;
pub const GRID_M: usize = 13;
#[derive(Default, Resource)]
pub struct Grid {
    grid: [[Option<Entity>; GRID_M]; GRID_N],
}

impl Grid {
    fn inside_grid(&self, pos: Vec2) -> bool {
        if pos.x >= GRID_N as f32 * SPRITE_SIZE || pos.x < 0. {
            return false;
        }
        if pos.y >= GRID_M as f32 * SPRITE_SIZE || pos.y < 0. {
            return false;
        }
        return true;
    }
    pub fn world_to_grid(&self, mut pos: Vec2) -> Option<UVec2> {
        pos += 10.5;
        if !self.inside_grid(pos) {
            return None;
        }

        let pos = uvec2((pos.x / SPRITE_SIZE) as u32, (pos.y / SPRITE_SIZE) as u32);
        if self.grid[pos.x as usize][pos.y as usize].is_none() {
            return Some(pos);
        }
        None
    }
}

pub fn init_grid(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in 0..GRID_N {
        for j in 0..GRID_M {
            commands.spawn((
                Sprite::from_image(asset_server.load("grid_cell.png")),
                Transform::from_translation(vec3(i as f32 * SPRITE_SIZE, j as f32 * SPRITE_SIZE, -1.)),
                Name::new("Grid_block"),
            ));
        }
    }
}

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InteractionPlugin);
        app.add_systems(Startup, init_grid);
        app.init_resource::<Grid>();
    }
}
