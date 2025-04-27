use bevy::prelude::*;

use crate::game::InGame;

pub mod cables;
pub mod packets;

#[derive(Component)]
#[require(InGame)]
pub struct PC;

#[derive(Component)]
#[require(InGame)]
pub struct Server;

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
pub struct Cable {
    pub dir: CableDirection,
}

#[derive(serde::Serialize, serde::Deserialize, Asset, TypePath, Copy, Clone)]
pub enum CableDirection {
    Vertical,
    Horizontal,
}

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(packets::PacketPlugin);
    }
}
