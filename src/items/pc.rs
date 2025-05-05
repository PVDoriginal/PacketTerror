use bevy::prelude::*;

use crate::{game::InGame, grid::Grid, health::UpdateHealthEvent};

use super::{
    packets::{EnemyPacket, Packet},
    projectiles::ProjectileType,
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
    enemy_packets: Query<(Entity, &Transform, &Packet), With<EnemyPacket>>,
    mut commands: Commands,
    pcs: Query<(&GlobalTransform, &PC)>,
    grid: ResMut<Grid>,
    mut update_health_writer: EventWriter<UpdateHealthEvent>,
) {
    for (packet_entity, pos, packet_enemy) in &enemy_packets {
        if let Some((_, _)) = grid
            .get_element(pos.translation.truncate())
            .and_then(|e| pcs.get(e).ok())
        {
            update_health_writer.send(UpdateHealthEvent(-packet_enemy.stats().damage));
            commands.entity(packet_entity).despawn();
        }
    }
}
