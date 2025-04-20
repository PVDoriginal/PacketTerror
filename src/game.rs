use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameStates {
    MainMenu,
    InGame,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameStates::MainMenu);
        app.add_systems(
            Update,
            main_menu_state.run_if(in_state(GameStates::MainMenu)),
        );
    }
}

fn main_menu_state(mut next_state: ResMut<NextState<GameStates>>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(GameStates::InGame);
    }
}
