use bevy::prelude::*;

use crate::{game::InGame, grid::Grid, health::update_health};

use super::{
    packets::{EnemyPacket, PlayerPacket},
    projectiles::{Projectile, ProjectileType},
};

#[derive(Component)]
#[require(InGame, ProjectileType)]
pub struct PC;

pub struct PcsPlugin;

impl Plugin for PcsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, take_damage);
    }
}

fn take_damage(
    player_packets: Query<(Entity, &Transform), With<PlayerPacket>>,
    enemy_packets: Query<(Entity, &Transform, &Packet), With<EnemyPacket>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    pcs: Query<(&GlobalTransform, &PC, &ProjectileType)>,
    grid: ResMut<Grid>,
) {
    for (packet_entity, pos) in &enemy_packets {
        if let Some((t_pc, _, &projectile_type)) = grid
            .get_element(pos.translation.truncate())
            .and_then(|e| pcs.get(e).ok())
        {
            if let Some((target, _)) = enemy_packets.iter().max_by(|&(_, t1), &(_, t2)| {
                t1.translation
                    .distance(pos.translation)
                    .total_cmp(&t2.translation.distance(pos.translation))
                    .reverse()
            }) {
                update_health_writer.send(UpdateHealthEvent(-packet_entity.stats().damage));
            }

            commands.entity(packet_entity).despawn();
        }
    }
}
