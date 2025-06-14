use bevy::prelude::*;

use crate::items::servers::{FireRate, Server};

use super::Upgradable;

impl Upgradable for Server {
    type Data = &'static mut FireRate;

    fn init_price(&self) -> i32 {
        10
    }

    fn upgrade(
        &self,
        level: u32,
        data: &mut bevy::ecs::query::QueryItem<Self::Data>,
    ) -> Option<i32> {
        let fire_rate = data;

        match level {
            0 => {
                fire_rate.0 = Timer::from_seconds(2., TimerMode::Repeating); //upgrading makes the server send packets faster
                Some(20) //upgrade cost
            }
            1 => {
                fire_rate.0 = Timer::from_seconds(1.5, TimerMode::Repeating);
                None
            }
            _ => None,
        }
    }
}
