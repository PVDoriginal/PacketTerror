use bevy::prelude::*;
use cables::CablesPlugin;
use enemy_pc::EnemyPCPlugin;
use packets::PacketsPlugin;
use projectiles::ProjectilePlugin;
use routers::RoutersPlugin;
use servers::ServersPlugin;
use switches::SwitchesPlugin;
use upgrades::UpgradesPlugin;

use crate::game::InGame;

pub mod cables;
pub mod enemy_pc;
pub mod packets;
pub mod projectiles;
pub mod routers;
pub mod servers;
pub mod switches;
pub mod upgrades;

#[derive(Component)]
#[require(InGame)]
pub struct PC;

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
            UpgradesPlugin,
        ));
    }
}
