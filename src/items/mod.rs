use bevy::prelude::*;

#[derive(Component)]
pub struct PC;

#[derive(Component)]
pub struct Router;

#[derive(Component)]
pub struct Switch;

#[derive(Component)]
pub struct Cable;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {}
}
