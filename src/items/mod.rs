use bevy::prelude::*;
use cables::CablesPlugin;
use enemy_pc::EnemyPCPlugin;
use packets::PacketsPlugin;
use pcs::PcsPlugin;
use projectiles::ProjectilePlugin;
use routers::RoutersPlugin;
use servers::ServersPlugin;
use switches::SwitchesPlugin;
use upgrades::UpgradesPlugin;

use crate::items::items_ui::ItemsUIPlugin;

pub mod cables;
pub mod enemy_pc;
pub mod items_ui;
pub mod packets;
pub mod pcs;
pub mod projectiles;
pub mod routers;
pub mod servers;
pub mod switches;
pub mod upgrades;

pub struct ItemsPlugin;

//adds all plugins so far
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
            PcsPlugin,
            ItemsUIPlugin,
        ));
    }
}
