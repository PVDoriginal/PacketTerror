use bevy::prelude::*;

use crate::game::InGame;

#[derive(Component, Clone)]
#[require(InGame)]
pub struct Packet {
    pub dir: Vec2,
    pub packet_type: PacketType,
}
impl Packet {
    pub fn new(dir: Vec2, packet_type: PacketType) -> Self {
        Self { dir, packet_type }
    }

    pub fn stats(&self) -> PacketStats {
        self.packet_type.into()
    }
}

#[derive(Clone, Copy)]
pub enum PacketType {
    Basic,
}

pub struct PacketStats {
    pub speed: f32,
    pub health: i32,
    pub damage: i32,
}

impl Into<PacketStats> for PacketType {
    fn into(self) -> PacketStats {
        match self {
            Self::Basic => PacketStats {
                speed: 10.,
                health: 10,
                damage: 5,
            },
        }
    }
}

#[derive(Component)]
#[require(InGame)]
pub struct EnemyPacket;

#[derive(Component)]
#[require(InGame)]
pub struct PlayerPacket;
