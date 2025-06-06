use bevy::prelude::*;

use crate::game::InGame;

#[derive(Component, Clone)]
#[require(InGame)]
pub struct Packet {
    pub dir: Vec2,
    pub hp: i32,
    pub packet_type: PacketType,
    pub dmg_multi: i32,
}
impl Packet {
    pub fn new(dir: Vec2, packet_type: PacketType) -> Self {
        let stats: PacketStats = packet_type.into();
        Self {
            dir,
            packet_type,
            hp: stats.health,
            dmg_multi: 1,
        }
    }

    pub fn stats(&self) -> PacketStats {
        self.packet_type.into()
    }
}

#[derive(Clone, Copy)]
pub enum PacketType {
    Basic,
    Mid,
    Advanced,
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
            Self::Mid => PacketStats {
                speed: 10.,
                health: 15,
                damage: 8,
            },
            Self::Advanced => PacketStats {
                speed: 10.,
                health: 25,
                damage: 11,
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

#[derive(Event)]
pub struct PacketDamageEvent {
    pub target: Entity,
    pub damage: i32,
}

pub struct PacketsPlugin;

impl Plugin for PacketsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PacketDamageEvent>();
        app.add_systems(Update, receive_damage);
    }
}

fn receive_damage(
    mut event: EventReader<PacketDamageEvent>,
    mut packets: Query<&mut Packet>,
    mut commands: Commands,
) {
    for e in event.read() {
        let Ok(mut packet) = packets.get_mut(e.target) else {
            continue;
        };

        packet.hp -= e.damage;
        if packet.hp <= 0 {
            commands.entity(e.target).try_despawn();
        }
    }
}
