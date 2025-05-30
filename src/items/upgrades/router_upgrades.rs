use crate::items::routers::{DamageMultiplier, Router};

use super::Upgradable;

impl Upgradable for Router {
    type Data = &'static mut DamageMultiplier;

    fn init_price(&self) -> i32 {
        10
    }

    fn upgrade(
        &self,
        level: u32,
        data: &mut bevy::ecs::query::QueryItem<Self::Data>,
    ) -> Option<i32> {
        let damage_multiply = data;

        match level {
            0 => {
                damage_multiply.0 = 1;
                Some(20)
            }
            1 => {
                damage_multiply.0 = 2;
                Some(30)
            }
            2 => {
                damage_multiply.0 = 3;
                None
            }
            _ => None,
        }
    }
}
