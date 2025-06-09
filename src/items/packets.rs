use std::time::Duration;

use bevy::prelude::*;

use crate::{camera::ScreenShake, game::InGame};

#[derive(Component)]
struct HitTime {
    timer: Timer,
}

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
        app.add_systems(Update, (receive_damage, packet_fx, packet_end_hit).chain());
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

fn packet_fx(
    mut event: EventReader<PacketDamageEvent>,
    packets: Query<&Packet>,
    mut sprites: Query<&mut Sprite>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,

    mut screen_shake: ResMut<ScreenShake>,
) {
    for e in event.read() {
        let Ok(packet) = packets.get(e.target) else {
            continue;
        };

        let Ok(mut sprite) = sprites.get_mut(e.target) else {
            continue;
        };

        if packet.hp <= 0 {
            screen_shake.shake(1., 0.1);
        } else {
            *sprite = Sprite::from_image(asset_server.load("white_packet.png"));

            commands.entity(e.target).insert(HitTime {
                timer: Timer::new(Duration::from_secs(1), TimerMode::Once),
            });
        }
    }
}

fn packet_end_hit(
    mut packets: Query<(&mut Sprite, &mut HitTime), With<Packet>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    for (mut sprite, mut hit_time) in packets.iter_mut() {
        hit_time.timer.tick(time.delta());

        if hit_time.timer.finished() {
            *sprite = Sprite::from_image(asset_server.load("enemy_packet.png"));
        }
    }
}
