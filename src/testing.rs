use bevy::prelude::*;

use crate::health::{Health, HealthPlugin};

#[derive(Resource)]
pub struct HealthTest {
    pub value: i32,
}

#[test]
fn test_health() {
    let mut app = App::new();
    app.insert_resource(HealthTest { value: 5 });
    app.add_plugins(HealthPlugin);

    app.add_systems(Startup, (set_health, is_health).chain());
}

fn set_health(mut health: ResMut<Health>, health_test: Res<HealthTest>) {
    health.value = health_test.value;
}

fn is_health(health: Res<Health>, health_test: Res<HealthTest>) {
    assert_eq!(health.value, health_test.value);
}
