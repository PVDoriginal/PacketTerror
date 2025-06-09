use bevy::prelude::*;

use crate::{
    camera::{SPRITE_SIZE, Shake},
    game::InGame,
    grid::Grid,
};

use super::{
    cables::{Cable, get_adj_cables},
    packets::{EnemyPacket, Packet, PlayerPacket},
};

#[derive(Component, Default)]
pub struct DamageMultiplier(pub i32);

#[derive(Component)]
#[require(InGame, DamageMultiplier)]
pub struct Router;

pub struct RoutersPlugin;

impl Plugin for RoutersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, redirect_packets);
    }
}

fn redirect_packets(
    mut packets: Query<(
        &mut Transform,
        &Sprite,
        &mut Packet,
        &Name,
        Entity,
        Option<&PlayerPacket>,
        Option<&EnemyPacket>,
    )>,
    routers: Query<(Entity, &DamageMultiplier, &Router)>,
    cables: Query<&Cable>,
    grid: ResMut<Grid>,
    mut commands: Commands,
) {
    for (mut pos, sprite, mut packet, name, packet_entity, is_player, is_enemy) in &mut packets {
        if let Some((router, r_dmg_multi, _)) = grid
            .get_element(pos.translation.truncate())
            .and_then(|e| routers.get(e).ok())
        {
            commands
                .entity(router)
                .insert(Shake::new(3., 0.2, pos.translation));
            let cables: Vec<(Vec2, Vec2)> =
                get_adj_cables(pos.translation.truncate(), &cables, &grid)
                    .into_iter()
                    .filter(|(_, adj_space)| adj_space * -1. != packet.dir)
                    .collect();

            if cables.len() == 0 {
                commands.entity(packet_entity).try_despawn();
                continue;
            }

            packet.dmg_multi = r_dmg_multi.0;

            for (index, &(cable_pos, adj_space)) in cables.iter().enumerate() {
                // move the last packet
                if index == cables.len() - 1 {
                    packet.dir = adj_space;
                    pos.translation = (cable_pos - adj_space * SPRITE_SIZE / 2.05).extend(2.);
                }
                // spawn new packets
                else {
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
            }
        }
    }
}
