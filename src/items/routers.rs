use bevy::prelude::*;

use crate::{camera::SPRITE_SIZE, grid::Grid};

use super::{
    Cable, Router,
    cables::get_adj_cables,
    packets::{EnemyPacket, Packet, PlayerPacket},
};

pub struct RoutersPlugin;

impl Plugin for RoutersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, redirect_packets);
    }
}

fn redirect_packets(
    packets: Query<(
        &Transform,
        &Sprite,
        &Packet,
        &Name,
        Entity,
        Option<&PlayerPacket>,
        Option<&EnemyPacket>,
    )>,
    routers: Query<&Router>,
    cables: Query<&Cable>,
    grid: ResMut<Grid>,
    mut commands: Commands,
) {
    for (pos, sprite, packet, name, packet_entity, is_player, is_enemy) in &packets {
        if let Some(_) = grid
            .get_element(pos.translation.truncate())
            .and_then(|e| routers.get(e).ok())
        {
            let cables = get_adj_cables(pos.translation.truncate(), &cables, &grid);

            for (cable_pos, adj_space) in cables {
                if adj_space * -1. == packet.dir {
                    continue;
                }

                let mut packet = packet.clone();
                packet.dir = adj_space;

                let mut new_packet = commands.spawn((
                    packet,
                    sprite.clone(),
                    name.clone(),
                    Transform::from_translation(
                        (cable_pos - adj_space * SPRITE_SIZE / 2.05).extend(2.),
                    ),
                ));

                is_player.map(|_| new_packet.insert(PlayerPacket));
                is_enemy.map(|_| new_packet.insert(EnemyPacket));
            }
            commands.entity(packet_entity).despawn();
        }
    }
}
