use bevy::prelude::*;
use cables::CablesPlugin;
use enemy_pc::EnemyPCPlugin;
use packets::PacketsPlugin;
use projectiles::ProjectilePlugin;
use routers::RoutersPlugin;
use servers::ServersPlugin;
use switches::SwitchesPlugin;

use crate::game::InGame;

pub mod cables;
pub mod enemy_pc;
pub mod packets;
pub mod projectiles;
pub mod routers;
pub mod servers;
pub mod switches;

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
        app.add_plugins((
            PacketsPlugin,
            CablesPlugin,
            EnemyPCPlugin,
            RoutersPlugin,
            ServersPlugin,
            SwitchesPlugin,
            ProjectilePlugin,
        ));
    }
}
