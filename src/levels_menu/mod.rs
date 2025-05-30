/*use bevy::prelude::*;
use ui::{EasyBtnPress, ExpertBtnPress, HardBtnPress, MediumBtnPress};

use crate::game::GameStates;

pub mod ui;

#[derive(Component)]
pub struct LevelsMenu;

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UIPlugin);
        app.add_systems(
            Update,
            (on_easy, on_medium, on_hard, on_expert).run_if(in_state(GameStates::LevelsMenu)),
        );
    }
}

fn on_easy(
    mut event: EventReader<PlayBtnPress>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut commands: Commands,
    main_menu: Query<Entity, With<MainMenu>>,
) {
    if event.read().len() == 0 {
        return;
    }

    let main_menu = main_menu.single();
    commands.entity(main_menu).despawn_recursive();

    next_state.set(GameStates::InGame);
}

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

fn on_quit(mut event: EventReader<QuitBtnPress>, mut exit: EventWriter<AppExit>) {
    if event.read().len() == 0 {
        return;
    }

    exit.send(AppExit::Success);
}
*/
