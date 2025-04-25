use std::time::Duration;

use crate::{camera::SPRITE_SIZE, game::GameStates, grid::Grid, items::EnemyPC};
use bevy::{math::vec2, prelude::*, time::common_conditions::on_timer};

use crate::game::InGame;

use super::{Cable, Server};

#[derive(Component)]
#[require(InGame)]
pub struct Packet {
    dir: Vec2,
    speed: f32,
}
impl Packet {
    pub fn new(dir: Vec2, speed: f32) -> Self {
        Self { dir, speed }
    }
}

pub struct PacketPlugin;

impl Plugin for PacketPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            create_packets
                .run_if(in_state(GameStates::InGame))
                .run_if(on_timer(Duration::from_secs(3))), // can I put them in a tuple?
        );
        app.add_systems(Update, update_packets.run_if(in_state(GameStates::InGame)));
    }
}

fn create_packets(
    packet_senders: Query<&Transform, Or<(With<EnemyPC>, With<Server>)>>,
    cables: Query<&Cable>,
    grid: ResMut<Grid>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for packet_sender in &packet_senders {
        let pos = grid
            .world_to_grid(packet_sender.translation.truncate())
            .expect("bad item position");
        const ADJ_SPACE: [Vec2; 4] = [vec2(-1., 0.), vec2(0., -1.), vec2(1., 0.), vec2(0., 1.)];

        for adj_space in ADJ_SPACE {
            let pos = (pos.as_vec2() + adj_space) * SPRITE_SIZE;
            let Some(entity) = grid.get_element(pos) else {
                continue;
            };
            if cables.get(entity).is_err() {
                continue;
            }

            commands.spawn((
                Packet::new(adj_space, 10.),
                Sprite::from_image(asset_server.load("packet.png")),
                Transform::from_translation((pos + adj_space * -SPRITE_SIZE / 2.05).extend(2.)),
                Name::from("Packet "),
            ));
        }
    }
}

fn update_packets(
    time: Res<Time>,
    mut packets: Query<(&mut Transform, &Packet)>,
    cables: Query<&Cable>,
    grid: ResMut<Grid>,
) {
    for (mut pos, packet) in packets.iter_mut() {
        let entity = grid
            .get_element(pos.translation.truncate())
            .expect("packet outside elements");

        if cables.get(entity).is_err() {
            continue;
        }

        pos.translation += packet.dir.extend(0.) * packet.speed * time.delta_secs();
    }
}
