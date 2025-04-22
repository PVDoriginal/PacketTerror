use bevy::prelude::*;
use interaction::InteractionPlugin;

pub mod interaction;

pub const GRID_SIZE: usize = 30;
#[derive(Default, Resource)]
pub struct Grid {
    grid: [[Option<Entity>; GRID_SIZE]; GRID_SIZE],
}

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InteractionPlugin);
    }
}
