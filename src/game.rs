use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameLevels {
    Sandbox,
    Easy,
    Medium,
    Hard,
    Expert,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameStates {
    Start,
    MainMenu,
    InGame,
    LevelsMenu,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuildStates {
    Release,
    Internal,
}

#[derive(Component, Default)]
pub struct InGame;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameStates::Start);
        app.insert_state(GameLevels::Sandbox);
        app.insert_state(BuildStates::Internal);

        app.add_systems(Update, start_state.run_if(in_state(GameStates::Start)));
        app.add_systems(Update, main_menu_on_escape);

        app.add_systems(OnExit(GameStates::InGame), despawn_game);
    }
}

// runs before entering main menu
fn start_state(mut next_state: ResMut<NextState<GameStates>>) {
    next_state.set(GameStates::MainMenu);
}

// runs when exiting game state
fn despawn_game(mut commands: Commands, game: Query<Entity, With<InGame>>) {
    for game_object in &game {
        commands.entity(game_object).try_despawn_recursive();
    }
}
fn main_menu_on_escape(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameStates>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameStates::MainMenu);
    }
}
