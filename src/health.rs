use bevy::prelude::*;

use crate::game::{GameStates, InGame};

#[derive(Resource)]
pub struct Health {
    pub value: i32,
}
#[derive(Component)]
#[require(InGame)]
pub struct HealthDisplay;

#[derive(Event)]
pub struct UpdateHealthEvent(pub i32);

pub struct HealthPlugin;
impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Health { value: 100 });
        app.add_systems(OnEnter(GameStates::InGame), init_health);
        app.add_systems(
            Update,
            (update_health, damage_test).run_if(in_state(GameStates::InGame)),
        );
        app.add_event::<UpdateHealthEvent>();
    }
}

pub fn init_health(mut commands: Commands, health: Res<Health>) {
    commands.spawn((
        HealthDisplay,
        Text::new(format!("Player Health: {} / 100", health.value)),
        TextFont {
            font_size: 14.0,
            ..Default::default()
        },
        Node {
            top: Val::Px(15.0),
            ..Default::default()
        },
    ));
}

pub fn update_health(
    mut health: ResMut<Health>,
    mut event_update: EventReader<UpdateHealthEvent>,
    mut display_health: Query<&mut Text, With<HealthDisplay>>,
) {
    for ev in event_update.read() {
        if health.value + ev.0 >= 0 {
            health.value += ev.0;
        }
    }

    let Ok(mut text) = display_health.get_single_mut() else {
        return;
    };
    text.0 = format!("Player Health: {} / 100", health.value);
}

pub fn damage_test(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut update_health_writer: EventWriter<UpdateHealthEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        update_health_writer.send(UpdateHealthEvent(-1));
    }
}
