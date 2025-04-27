use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::{
    camera::SPRITE_SIZE,
    game::GameStates,
    grid::Grid,
    items::{Cable, EnemyPC, Server},
};

use super::{EnemyPacket, Packet, PacketType, PlayerPacket, util::get_adj_cables};

pub struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (create_packets_enemy_pc, create_packets_server)
                .run_if(in_state(GameStates::InGame))
                .run_if(on_timer(Duration::from_secs(3))),
        );
    }
}
pub fn create_packets_enemy_pc(
    packet_senders: Query<&Transform, With<EnemyPC>>,
    cables: Query<&Cable>,
    grid: ResMut<Grid>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for packet_sender in &packet_senders {
        let cables = get_adj_cables(packet_sender.translation.truncate(), &cables, &grid);

        for (cable_pos, adj_space) in cables {
            commands.spawn((
                EnemyPacket,
                Packet::new(adj_space, PacketType::Basic),
                Sprite::from_image(asset_server.load("enemy_packet.png")),
                Transform::from_translation(
                    (cable_pos - adj_space * SPRITE_SIZE / 2.05).extend(2.),
                ),
                Name::from("Enemy packet"),
            ));
        }
    }
}

pub fn create_packets_server(
    packet_senders: Query<&Transform, With<Server>>,
    cables: Query<&Cable>,
    grid: ResMut<Grid>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for packet_sender in &packet_senders {
        let cables = get_adj_cables(packet_sender.translation.truncate(), &cables, &grid);

        for (cable_pos, adj_space) in cables {
            commands.spawn((
                PlayerPacket,
                Packet::new(adj_space, PacketType::Basic),
                Sprite::from_image(asset_server.load("player_packet.png")),
                Transform::from_translation(
                    (cable_pos - adj_space * SPRITE_SIZE / 2.05).extend(2.),
                ),
                Name::from("Player packet"),
            ));
        }
    }
}
