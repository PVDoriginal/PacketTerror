use bevy::prelude::*;

use crate::items::{projectiles::ProjectileType, switches::Switch};

use super::Upgradable;

impl Upgradable for Switch {
    type Data = &'static mut ProjectileType;

    fn init_price(&self) -> i32 {
        15
    }

    fn upgrade(
        &self,
        level: u32,
        data: &mut bevy::ecs::query::QueryItem<Self::Data>,
    ) -> Option<i32> {
        let projectile_type = data;

        match level {
            0 => {
                projectile_type.set_if_neq(ProjectileType::Mid);
                Some(20)
            }
            1 => {
                projectile_type.set_if_neq(ProjectileType::Advanced);
                None
            }
            _ => None,
        }
    }
}
