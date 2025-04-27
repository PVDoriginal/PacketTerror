use bevy::{prelude::*, utils::info};

use crate::{
    camera::SPRITE_SIZE,
    game::GameStates,
    grid::Grid,
    items::{Cable, Router, Switch},
};

use super::{EnemyPacket, Packet, PlayerPacket, util::get_adj_cables};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_packets.run_if(in_state(GameStates::InGame)));
    }
}

fn move_packets(
    time: Res<Time>,
    mut packets: Query<(
        &mut Transform,
        &Sprite,
        &Packet,
        &Name,
        Entity,
        Option<&PlayerPacket>,
        Option<&EnemyPacket>,
    )>,
    router: Query<&Router>,
    cables: Query<&Cable>,
    switches: Query<&Switch>,
    grid: ResMut<Grid>,
    mut commands: Commands,
) {
    for (mut pos, sprite, packet, name, packet_entity, is_player, is_enemy) in packets.iter_mut() {
        let Some(entity) = grid.get_element(pos.translation.truncate()) else {
            info!("TF?");
            continue;
        };

        if cables.get(entity).is_ok() {
            pos.translation += packet.dir.extend(0.) * packet.stats().speed * time.delta_secs();
            continue;
        }

        if router.get(entity).is_ok() {
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
            continue;
        }
        commands.entity(packet_entity).despawn();
    }
}
