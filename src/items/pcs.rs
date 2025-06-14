use bevy::prelude::*;

use crate::{game::InGame, grid::Grid, health::UpdateHealthEvent, shake::Shake};

use super::packets::{EnemyPacket, Packet};

#[derive(Component)]
#[require(InGame)]
pub struct PC;

pub struct PcsPlugin;

impl Plugin for PcsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, take_damage);
    }
}

//loses health when an enemy packet reaches it
fn take_damage(
    enemy_packets: Query<(Entity, &Transform, &Packet), With<EnemyPacket>>,
    mut commands: Commands,
    pcs: Query<&PC>,
    cameras: Query<(Entity, &Transform), With<Camera2d>>,
    grid: Res<Grid>,
    mut update_health_writer: EventWriter<UpdateHealthEvent>,
) {
    for (packet_entity, pos, packet) in &enemy_packets {
        if let Some(_) = grid
            .get_element(pos.translation.truncate())
            .and_then(|e| pcs.get(e).ok())
        {
            update_health_writer.send(UpdateHealthEvent(-packet.stats().damage));
            let Ok((camera, pos)) = cameras.get_single() else {
                return;
            };

            commands
                .entity(camera)
                .insert_if_new(Shake::new(15., 0.2, pos.translation));

            commands.entity(packet_entity).despawn();
        }
    }
}
