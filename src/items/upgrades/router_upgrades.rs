use bevy::prelude::*;

use crate::items::routers::{DamageMultiply, Router};

use super::Upgradable;

impl Upgradable for Router {
    type Data = &'static mut DamageMultiply;

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
                damage_multiply.0 *= 1.5;
                Some(20)
            }
            1 => {
                damage_multiply.0 *= 2;
                None
            }
            2 => {
                damage_multiply.0 *= 2.5;
                None
            }
            _ => None,
        }
    }
}
