use bevy::prelude::*;

use crate::{game::InGame, grid::Grid, health::UpdateHealthEvent};

use super::{
    packets::{EnemyPacket, Packet, PlayerPacket},
    projectiles::{Projectile, ProjectileType},
};

#[derive(Component)]
#[require(InGame)]
pub struct PC;

pub struct PcsPlugin;

impl Plugin for PcsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, take_damage);
    }
}

fn take_damage(
    enemy_packets: Query<(Entity, &Transform, &Packet), With<EnemyPacket>>,
    mut commands: Commands,
    pcs: Query<&PC>,
    grid: Res<Grid>,
    mut update_health_writer: EventWriter<UpdateHealthEvent>,
) {
    for (packet_entity, pos, packet) in &enemy_packets {
        if let Some(_) = grid
            .get_element(pos.translation.truncate())
            .and_then(|e| pcs.get(e).ok())
        {
            update_health_writer.send(UpdateHealthEvent(-packet.stats().damage));
            commands.entity(packet_entity).despawn();
        }
    }
}
