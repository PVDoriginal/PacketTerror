use std::time::Duration;

use bevy::{
    ecs::query::{QueryData, QueryItem},
    prelude::*,
};

use crate::{camera::SPRITE_SIZE, shop::currency::Currency};

use super::{routers::Router, servers::Server, switches::Switch};

pub mod router_upgrades;
pub mod server_upgrades;
pub mod switch_upgrades;

pub const UPGRADE_TIME: f32 = 1.0;

#[derive(Resource)]
pub struct UpgradeTimer {
    timer: Timer,
    entity: Option<Entity>,
}
#[derive(Component)]
pub struct Upgrading;

pub struct UpgradesPlugin;

impl Plugin for UpgradesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UpgradeTimer {
            timer: Timer::new(Duration::from_secs_f32(0.), TimerMode::Once),
            entity: None,
        });

        app.add_systems(
            Update,
            (
                init_upgrades::<Server>,
                init_upgrades::<Switch>,
                init_upgrades::<Router>,
                upgrade::<Server>,
                upgrade::<Switch>,
                upgrade::<Router>,
            ),
        );
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
    pub next_price: Option<i32>,
}

impl From<i32> for UpgradeLevel {
    fn from(next_price: i32) -> Self {
        UpgradeLevel {
            level: 0,
            next_price: Some(next_price),
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
            .observe(start_upgrade::<T>)
            .observe(end_upgrade::<T>);
    }
}

fn start_upgrade<T: Upgradable + Component>(
    trigger: Trigger<Pointer<Down>>,
    mut items: Query<(&Transform, &mut UpgradeLevel)>,
    currency: ResMut<Currency>,
    mut upgrade_timer: ResMut<UpgradeTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let Ok((pos, level)) = items.get_mut(trigger.entity()) else {
        return;
    };
    let Some(price) = level.next_price else {
        return;
    };
    if currency.value < price {
        return;
    }

    upgrade_timer.timer = Timer::new(Duration::from_secs_f32(UPGRADE_TIME), TimerMode::Once);
    upgrade_timer.entity = Some(trigger.entity());

    let mut position = pos.translation;
    position.z += 1.;

    commands.spawn((
        Upgrading,
        Transform::from_translation(position),
        MeshMaterial2d(materials.add(Color::Srgba(Srgba::new(1., 1., 1., 0.15)))),
        Mesh2d(meshes.add(Rectangle::new(SPRITE_SIZE, 0.))),
    ));
}

fn end_upgrade<T: Upgradable + Component>(
    _: Trigger<Pointer<Up>>,
    mut commands: Commands,
    mut rect: Query<Entity, With<Upgrading>>,
) {
    for rectangle in rect.iter_mut() {
        commands.entity(rectangle).despawn();
    }
}

fn upgrade<T: Upgradable + Component>(
    mut items: Query<(&T, &mut UpgradeLevel, T::Data)>,
    time: Res<Time>,
    mut currency: ResMut<Currency>,
    mut upgrade_timer: ResMut<UpgradeTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut rect: Query<&mut Mesh2d, With<Upgrading>>,
) {
    if upgrade_timer.timer.finished() {
        return;
    }

    let Some(entity) = upgrade_timer.entity else {
        return;
    };
    let Ok((item, mut level, mut data)) = items.get_mut(entity) else {
        return;
    };

    let Some(price) = level.next_price else {
        return;
    };
    if currency.value < price {
        return;
    }

    upgrade_timer.timer.tick(time.delta());

    for mut rectangle in rect.iter_mut() {
        **rectangle = meshes.add(Rectangle::new(
            SPRITE_SIZE,
            upgrade_timer.timer.elapsed_secs() / UPGRADE_TIME * SPRITE_SIZE,
        ));
    }

    if upgrade_timer.timer.just_finished() {
        currency.value -= price;

        level.next_price = item.upgrade(level.level, &mut data);
        level.level += 1;
    }
}
