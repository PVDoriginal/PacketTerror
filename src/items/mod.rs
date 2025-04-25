use bevy::prelude::*;

use crate::game::InGame;

#[derive(Component)]
#[require(InGame)]
pub struct PC;

#[derive(Component)]
#[require(InGame)]
pub struct EnemyPC;

#[derive(Component)]
#[require(InGame)]
pub struct Router;

#[derive(Component)]
#[require(InGame)]
pub struct Switch;

#[derive(Component)]
#[require(InGame)]
pub struct Cable;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {}
}
