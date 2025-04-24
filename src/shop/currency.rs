use bevy::prelude::*;

#[derive(Resource)]
pub struct Currency {
    pub value: i32,
}
#[derive(Component)]
pub struct CurrencyDisplay;

#[derive(Event)]
pub struct UpdateCurrencyEvent(pub i32);

pub fn init_currency(mut commands: Commands, currency: Res<Currency>) {
    // Note: text without textBundle seems to float to screen top-left
    commands.spawn((
        CurrencyDisplay,
        Text::new(format!("Packet credits: {}", currency.value)),
        TextFont {
            font_size: 14.0,
            ..Default::default()
        },
    ));
}
pub struct CurrencyPlugin;
impl Plugin for CurrencyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Currency { value: 300 });
        app.add_systems(Startup, init_currency);
        app.add_systems(Update, update_currency);
    }
}
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
