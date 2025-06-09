use bevy::prelude::*;
use ui::{BackBtnPress, EasyBtnPress, ExpertBtnPress, HardBtnPress, MediumBtnPress};

use crate::game::{GameLevels, GameStates};
use crate::levels_menu::ui::LevUIPlugin;

pub mod ui;

#[derive(Component)]
pub struct LevelsMenu;

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LevUIPlugin);
        app.add_systems(
            Update,
            (on_easy, on_medium, on_hard, on_expert, on_back)
                .run_if(in_state(GameStates::LevelsMenu)),
        );
        app.add_systems(OnExit(GameStates::LevelsMenu), despawn_levels);
    }
}

fn despawn_levels(mut commands: Commands, levels_menu: Query<Entity, With<LevelsMenu>>) {
    let levels_menu = levels_menu.single();
    commands.entity(levels_menu).despawn_recursive();
}

fn on_easy(
    mut event: EventReader<EasyBtnPress>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut next_level: ResMut<NextState<GameLevels>>,
) {
    if event.read().len() == 0 {
        return;
    }

    next_level.set(GameLevels::Easy);
    next_state.set(GameStates::InGame);
}

fn on_medium(
    mut event: EventReader<MediumBtnPress>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut next_level: ResMut<NextState<GameLevels>>,
) {
    if event.read().len() == 0 {
        return;
    }

    next_level.set(GameLevels::Medium);
    next_state.set(GameStates::InGame);
}

fn on_hard(
    mut event: EventReader<HardBtnPress>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut next_level: ResMut<NextState<GameLevels>>,
) {
    if event.read().len() == 0 {
        return;
    }

    next_level.set(GameLevels::Hard);
    next_state.set(GameStates::InGame);
}

fn on_expert(
    mut event: EventReader<ExpertBtnPress>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut next_level: ResMut<NextState<GameLevels>>,
) {
    if event.read().len() == 0 {
        return;
    }

    next_level.set(GameLevels::Expert);
    next_state.set(GameStates::InGame);
}

fn on_back(mut event: EventReader<BackBtnPress>, mut next_state: ResMut<NextState<GameStates>>) {
    if event.read().len() == 0 {
        return;
    }

    next_state.set(GameStates::MainMenu);
}
