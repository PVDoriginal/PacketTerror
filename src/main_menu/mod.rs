use bevy::prelude::*;
use ui::{LevelsBtnPress, PlayBtnPress, QuitBtnPress, UIPlugin};

use crate::game::{GameLevels, GameStates};

pub mod ui;

#[derive(Component)]
pub struct MainMenu;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UIPlugin);
        app.add_systems(
            Update,
            (on_play, on_levels, on_quit).run_if(in_state(GameStates::MainMenu)),
        );
    }
}

//sends you to sandbox
fn on_play(
    mut event: EventReader<PlayBtnPress>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut commands: Commands,
    main_menu: Query<Entity, With<MainMenu>>,
    mut next_level: ResMut<NextState<GameLevels>>,
) {
    if event.read().len() == 0 {
        return;
    }
    let main_menu = main_menu.single();
    commands.entity(main_menu).despawn_recursive();

    next_level.set(GameLevels::Sandbox);
    next_state.set(GameStates::InGame);
}

//sends you to the levels menu
fn on_levels(
    mut event: EventReader<LevelsBtnPress>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut commands: Commands,
    main_menu: Query<Entity, With<MainMenu>>,
) {
    if event.read().len() == 0 {
        return;
    }

    let main_menu = main_menu.single();
    commands.entity(main_menu).despawn_recursive();

    next_state.set(GameStates::LevelsMenu);
}

//exits the game
fn on_quit(mut event: EventReader<QuitBtnPress>, mut exit: EventWriter<AppExit>) {
    if event.read().len() == 0 {
        return;
    }

    exit.send(AppExit::Success);
}
