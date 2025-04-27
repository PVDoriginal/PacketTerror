use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::{camera::SPRITE_SIZE, game::GameStates, grid::Grid};

use super::{
    Cable, Server,
    cables::get_adj_cables,
    packets::{Packet, PacketType, PlayerPacket},
};

pub struct ServersPlugin;

impl Plugin for ServersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            create_packets
                .run_if(in_state(GameStates::InGame))
                .run_if(on_timer(Duration::from_secs(3))),
        );
    }
}

fn create_packets(
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
