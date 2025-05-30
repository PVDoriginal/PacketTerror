use bevy::prelude::*;

use crate::{game::InGame, grid::Grid};

use super::{
    packets::{EnemyPacket, Packet, PlayerPacket},
    projectiles::{Projectile, ProjectileType},
};

#[derive(Component)]
#[require(InGame, ProjectileType)]
pub struct Switch;

pub struct SwitchesPlugin;

impl Plugin for SwitchesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, shoot_projectiles);
    }
}

fn shoot_projectiles(
    player_packets: Query<(Entity, &Transform, &Packet), With<PlayerPacket>>,
    enemy_packets: Query<(Entity, &Transform), With<EnemyPacket>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    switches: Query<(&GlobalTransform, &Switch, &ProjectileType)>,
    grid: ResMut<Grid>,
) {
    for (packet_entity, pos, packet) in &player_packets {
        if let Some((t_switch, _, &projectile_type)) = grid
            .get_element(pos.translation.truncate())
            .and_then(|e| switches.get(e).ok())
        {
            if let Some((target, _)) = enemy_packets.iter().max_by(|&(_, t1), &(_, t2)| {
                t1.translation
                    .distance(pos.translation)
                    .total_cmp(&t2.translation.distance(pos.translation))
                    .reverse()
            }) {
                commands.spawn((
                    Projectile {
                        target,
                        projectile_type,
                        dmg_multi: packet.dmg_multi,
                    },
                    Sprite::from_image(asset_server.load("projectile.png")),
                    Transform::from_translation(t_switch.translation()),
                ));
            }
            commands.entity(packet_entity).despawn();
        }
    }
}
