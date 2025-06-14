use bevy::prelude::*;

use crate::shake::Shake;

use super::packets::{EnemyPacket, PacketDamageEvent};

const COLLISION_RANGE: f32 = 1.;

#[derive(Component)]
pub struct Projectile {
    pub target: Entity,
    pub projectile_type: ProjectileType,
    pub dmg_multi: i32,
}

impl Projectile {
    pub fn stats(&self) -> ProjectileStats {
        self.projectile_type.into()
    }
}

pub struct ProjectileStats {
    pub speed: f32,
    pub damage: i32,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum ProjectileType {
    Basic,
    Mid,
    Advanced,
}

impl Default for ProjectileType {
    fn default() -> Self {
        Self::Basic
    }
}

//types of projectiles
impl Into<ProjectileStats> for ProjectileType {
    fn into(self) -> ProjectileStats {
        match self {
            Self::Basic => ProjectileStats {
                speed: 50.,
                damage: 6,
            },
            Self::Mid => ProjectileStats {
                speed: 60.,
                damage: 10,
            },
            Self::Advanced => ProjectileStats {
                speed: 100.,
                damage: 15,
            },
        }
    }
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_projectiles, collide));
    }
}

//travelling to the enemy packet
fn move_projectiles(
    mut projectiles: Query<(&mut Transform, &Projectile, Entity), Without<EnemyPacket>>,
    enemy_packets: Query<&Transform, With<EnemyPacket>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (mut t_projectile, projectile, projectile_id) in &mut projectiles {
        let Ok(t_target) = enemy_packets.get(projectile.target) else {
            commands.entity(projectile_id).despawn();
            continue;
        };

        let dir = (t_target.translation - t_projectile.translation).normalize();
        t_projectile.translation += dir * projectile.stats().speed * time.delta_secs();
    }
}

//projectile hits enemy packet
fn collide(
    projectiles: Query<(Entity, &GlobalTransform, &Projectile)>,
    enemy_packets: Query<(Entity, &GlobalTransform), With<EnemyPacket>>,
    mut damage_event: EventWriter<PacketDamageEvent>,
    cameras: Query<(Entity, &Transform), With<Camera2d>>,
    mut commands: Commands,
) {
    for (projectile_id, t_projectile, projectile) in &projectiles {
        let Ok((target, t_target)) = enemy_packets.get(projectile.target) else {
            continue;
        };

        if t_target.translation().distance(t_projectile.translation()) <= COLLISION_RANGE {
            damage_event.send(PacketDamageEvent {
                target,
                damage: projectile.stats().damage * projectile.dmg_multi,
            });
            commands.entity(projectile_id).despawn();

            let Ok((camera, pos)) = cameras.get_single() else {
                return;
            };
            commands
                .entity(camera)
                .insert(Shake::new(10., 0.02, pos.translation));
        }
    }
}
