use crate::{camera::SPRITE_SIZE, game::InGame, grid::Grid};
use bevy::{math::vec2, prelude::*};

use super::packets::Packet;

#[derive(Component)]
#[require(InGame)]
pub struct Cable {
    pub dir: CableDirection,
}

#[derive(serde::Serialize, serde::Deserialize, Asset, TypePath, Copy, Clone, Debug)]
pub enum CableDirection {
    Vertical,
    Horizontal,
}

pub struct CablesPlugin;

impl Plugin for CablesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_packets);
    }
}

fn move_packets(
    time: Res<Time>,
    mut packets: Query<(&mut Transform, &Packet)>,
    cables: Query<&Cable>,
    grid: ResMut<Grid>,
) {
    for (mut pos, packet) in packets.iter_mut() {
        if let Some(_) = grid
            .get_element(pos.translation.truncate())
            .and_then(|e| cables.get(e).ok())
        {
            pos.translation += packet.dir.extend(0.) * packet.stats().speed * time.delta_secs();
        }
    }
}

const ADJ_SPACE: [Vec2; 4] = [vec2(-1., 0.), vec2(0., -1.), vec2(1., 0.), vec2(0., 1.)];

pub fn get_adj_cables(
    start_pos: Vec2,
    cables: &Query<&Cable>,
    grid: &ResMut<Grid>,
) -> Vec<(Vec2, Vec2)> {
    let mut res: Vec<(Vec2, Vec2)> = Vec::new();

    let pos = grid.world_to_grid(start_pos).expect("bad item position");

    for adj_space in ADJ_SPACE {
        let pos = (pos.as_vec2() + adj_space) * SPRITE_SIZE;
        let Some(entity) = grid.get_element(pos) else {
            continue;
        };
        let Ok(cable) = cables.get(entity) else {
            continue;
        };

        match cable.dir {
            CableDirection::Horizontal => {
                if adj_space.y != 0. {
                    continue;
                }
            }
            CableDirection::Vertical => {
                if adj_space.x != 0. {
                    continue;
                }
            }
        };
        res.push((pos, adj_space));
    }
    res
}
