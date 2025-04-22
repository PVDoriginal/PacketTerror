use bevy::prelude::*;

pub const GRID_SIZE: usize = 30;
#[derive(Default, Resource)]
pub struct Grid {
    grid: [[Option<Entity>; GRID_SIZE]; GRID_SIZE],
}
