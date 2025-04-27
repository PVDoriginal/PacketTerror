use std::time::Duration;

use crate::{
    camera::SPRITE_SIZE,
    game::{GameStates, InGame},
    grid::Grid,
};
use bevy::{prelude::*, time::common_conditions::on_timer};

use super::{
    cables::{Cable, get_adj_cables},
    packets::{EnemyPacket, Packet, PacketType},
};

#[derive(Component)]
#[require(InGame)]
pub struct EnemyPC;

pub struct EnemyPCPlugin;

impl Plugin for EnemyPCPlugin {
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
