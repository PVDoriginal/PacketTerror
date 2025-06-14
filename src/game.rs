use std::{fs::File, io::Read};

use bevy::prelude::*;

use crate::{grid::Grid, health::Health, shop::currency::Currency};

pub const HIGHEST_LEVEL_PATH: &str = "assets/highestlvl";

#[derive(serde::Serialize, serde::Deserialize, Resource)]
pub struct HighestLevel {
    pub highest: GameLevels,
}

#[derive(serde::Serialize, serde::Deserialize, States, Debug, Clone, PartialEq, Eq, Hash, Copy)]
#[serde(into = "u8", from = "u8")]
pub enum GameLevels {
    Sandbox,
    Easy,
    Medium,
    Hard,
    Expert,
}
impl From<GameLevels> for u8 {
    fn from(value: GameLevels) -> u8 {
        value as u8
    }
}

//all the possible levels
impl From<u8> for GameLevels {
    fn from(value: u8) -> GameLevels {
        match value {
            0 => GameLevels::Sandbox,
            1 => GameLevels::Easy,
            2 => GameLevels::Medium,
            3 => GameLevels::Hard,
            4 => GameLevels::Expert,
            _ => panic!("Invalid value for GameLevels: {}", value),
        }
    }
}

//grids for each level
impl GameLevels {
    pub fn level_path(&self) -> String {
        let mut s1 = String::from("grids/");
        s1.push_str(match self {
            GameLevels::Sandbox => "sandbox.grid.json",
            GameLevels::Easy => "easy.grid.json",
            GameLevels::Medium => "medium.grid.json",
            GameLevels::Hard => "hard.grid.json",
            GameLevels::Expert => "expert.grid.json",
        });
        s1
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameStates {
    Start,
    MainMenu,
    InGame,
    LevelsMenu,
    DefeatScreen,
    VictoryScreen,
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
        app.insert_resource(HighestLevel {
            highest: GameLevels::Easy,
        });
        app.insert_state(GameStates::Start);
        app.insert_state(GameLevels::Sandbox);
        app.insert_state(BuildStates::Release);

        app.add_systems(Update, start_state.run_if(in_state(GameStates::Start)));
        app.add_systems(Update, main_menu_on_escape);

        app.add_systems(OnExit(GameStates::InGame), despawn_game);
    }
}

// runs before entering main menu
fn start_state(mut next_state: ResMut<NextState<GameStates>>, mut highest: ResMut<HighestLevel>) {
    next_state.set(GameStates::MainMenu);

    let Ok(mut file) = File::open(HIGHEST_LEVEL_PATH) else {
        return;
    };
    let mut contents = Vec::<u8>::new();
    file.read_to_end(&mut contents).ok();

    highest.highest = contents[0].into();
}

// runs when exiting game state
fn despawn_game(
    mut commands: Commands,
    game: Query<Entity, With<InGame>>,
    mut grid: ResMut<Grid>,
    mut health: ResMut<Health>,
    mut currency: ResMut<Currency>,
) {
    for game_object in &game {
        commands.entity(game_object).try_despawn_recursive();
    }
    grid.reset();
    health.value = 100;
    currency.value = 300;
}
fn main_menu_on_escape(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameStates>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameStates::MainMenu);
    }
}
