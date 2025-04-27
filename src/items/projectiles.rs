use bevy::prelude::*;

use crate::shop::shop_items::ShopPosition;

use super::packets::{EnemyPacket, Packet};

#[derive(Component)]
pub struct Projectile {
    pub target: Entity,
    pub projectile_type: ProjectileType,
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

#[derive(Clone, Copy)]
pub enum ProjectileType {
    Basic,
}

impl Into<ProjectileStats> for ProjectileType {
    fn into(self) -> ProjectileStats {
        match self {
            Self::Basic => ProjectileStats {
                speed: 50.,
                damage: 5,
            },
        }
    }
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_projectiles);
    }
}

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
