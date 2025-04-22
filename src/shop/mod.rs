use bevy::prelude::*;

pub struct ShopPlugin;

#[derive(Resource)]
pub struct Currency {
    value: i32,
}

#[derive(Event)]
pub struct UpdateCurrencyEvent(i32);

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Currency { value: 0 });
        app.add_systems(Update, update_currency);
        app.add_event::<UpdateCurrencyEvent>();
    }
}

pub fn update_currency(mut currency: ResMut<Currency>, mut event_remove : EventReader<UpdateCurrencyEvent>) {
    for ev in  event_remove.read(){
        currency.value += ev.0;
    }
}
