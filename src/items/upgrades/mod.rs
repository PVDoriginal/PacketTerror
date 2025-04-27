use bevy::{
    ecs::query::{QueryData, QueryItem},
    prelude::*,
};

use super::{servers::Server, switches::Switch};

pub mod server_upgrades;
pub mod switch_upgrades;

pub struct UpgradesPlugin;

impl Plugin for UpgradesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (init_upgrades::<Server>, init_upgrades::<Switch>));
    }
}

pub trait Upgradable {
    type Data: QueryData;

    // initial upgrade price
    fn init_price(&self) -> i32;

    // upgrades the item to this level, returns the cost for the next upgrade, if there is one
    fn upgrade(&self, level: u32, data: &mut QueryItem<Self::Data>) -> Option<i32>;
}

#[derive(Component)]
pub struct UpgradeLevel {
    pub level: u32,
    pub next_price: i32,
}

impl From<i32> for UpgradeLevel {
    fn from(next_price: i32) -> Self {
        UpgradeLevel {
            level: 0,
            next_price,
        }
    }
}

fn init_upgrades<T: Upgradable + Component>(
    items: Query<(Entity, &T), Added<T>>,
    mut commands: Commands,
) {
    for (id, item) in &items {
        commands
            .entity(id)
            .insert(UpgradeLevel::from(item.init_price()))
            .observe(upgrade::<T>);
    }
}

fn upgrade<T: Upgradable + Component>(
    trigger: Trigger<Pointer<Click>>,
    mut items: Query<(&T, &mut UpgradeLevel, T::Data)>,
) {
    let Ok((item, mut level, mut data)) = items.get_mut(trigger.entity()) else {
        return;
    };

    item.upgrade(level.level, &mut data);
    level.level += 1;
}
