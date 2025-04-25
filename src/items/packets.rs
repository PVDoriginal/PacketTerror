use crate::{grid::Grid, items::EnemyPC};
use bevy::{math::vec2, prelude::*};

use super::{Cable, Server};

pub struct PacketPlugin;

impl Plugin for PacketPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, create_packets);
    }
}

pub fn create_packets(
    packet_senders: Query<&Transform, Or<(With<EnemyPC>, With<Server>)>>,
    cables: Query<&Cable>,
    mut grid: ResMut<Grid>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for packet_sender in &packet_senders {
        let pos = grid
            .world_to_grid(packet_sender.translation.truncate())
            .expect("bad item position");
        const ADJ_SPACE: [Vec2; 4] = [vec2(-1., 0.), vec2(0., -1.), vec2(1., 0.), vec2(0., 1.)];
        for adj_space in ADJ_SPACE {
            let pos = pos.as_vec2() + adj_space;
            let Some(entity) = grid.get_element(pos) else {
                continue;
            };
            if cables.get(entity).is_err() {
                continue;
            }

            commands.spawn((
                Sprite::from_image(asset_server.load("packet.png")),
                Transform::from_translation(pos.extend(0.5)),
            ));
        }
    }
}
