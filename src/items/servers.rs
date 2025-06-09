use bevy::prelude::*;

use crate::{
    camera::{SPRITE_SIZE, Shake},
    game::{GameStates, InGame},
    grid::Grid,
};

use super::{
    cables::{Cable, get_adj_cables},
    packets::{Packet, PacketType, PlayerPacket},
};

#[derive(Component)]
pub struct FireRate(pub Timer);
impl Default for FireRate {
    fn default() -> Self {
        FireRate(Timer::from_seconds(3., TimerMode::Repeating))
    }
}

#[derive(Component)]
#[require(InGame, FireRate)]
pub struct Server;

pub struct ServersPlugin;

impl Plugin for ServersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, create_packets.run_if(in_state(GameStates::InGame)));
    }
}

fn create_packets(
    mut packet_senders: Query<(Entity, &Transform, &mut FireRate), With<Server>>,
    cables: Query<&Cable>,
    grid: ResMut<Grid>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    for (server_entity, packet_sender, mut fire_rate) in &mut packet_senders {
        if !fire_rate.0.tick(time.delta()).just_finished() {
            continue;
        }
        commands
            .entity(server_entity)
            .insert(Shake::new(2., 0.1, packet_sender.translation));

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
