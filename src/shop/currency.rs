use bevy::prelude::*;

use crate::game::{GameStates, InGame};

#[derive(Resource)]
pub struct Currency {
    pub value: i32,
}
#[derive(Component)]
#[require(InGame)]
pub struct CurrencyDisplay;

#[derive(Event)]
pub struct UpdateCurrencyEvent(pub i32);

pub struct CurrencyPlugin;
impl Plugin for CurrencyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Currency { value: 300 });
        app.add_systems(OnEnter(GameStates::InGame), init_currency);
        app.add_systems(Update, update_currency.run_if(in_state(GameStates::InGame)));
    }
}

//show the currency
pub fn init_currency(mut commands: Commands, currency: Res<Currency>) {
    commands.spawn((
        CurrencyDisplay,
        Text::new(format!("Packet credits: {}", currency.value)),
        TextFont {
            font_size: 14.0,
            ..Default::default()
        },
    ));
}

//lose it when you buy something
pub fn update_currency(
    mut currency: ResMut<Currency>,
    mut event_update: EventReader<UpdateCurrencyEvent>,
    mut display_currency: Query<&mut Text, With<CurrencyDisplay>>,
) {
    for ev in event_update.read() {
        if currency.value + ev.0 >= 0 {
            currency.value += ev.0;
        }
    }

    let Ok(mut text) = display_currency.get_single_mut() else {
        return;
    };
    text.0 = format!("Packet credits: {}", currency.value);
}
