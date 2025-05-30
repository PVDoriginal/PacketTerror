use bevy::prelude::*;
use ui::{BackBtnPress, EasyBtnPress, ExpertBtnPress, HardBtnPress, MediumBtnPress};

use crate::game::GameStates;
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
    }
}

fn on_easy(
    mut event: EventReader<EasyBtnPress>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut commands: Commands,
    levels_menu: Query<Entity, With<LevelsMenu>>,
) {
    if event.read().len() == 0 {
        return;
    }

    let levels_menu = levels_menu.single();
    commands.entity(levels_menu).despawn_recursive();

    next_state.set(GameStates::Easy);
}

fn on_medium(
    mut event: EventReader<MediumBtnPress>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut commands: Commands,
    levels_menu: Query<Entity, With<LevelsMenu>>,
) {
    if event.read().len() == 0 {
        return;
    }

    let levels_menu = levels_menu.single();
    commands.entity(levels_menu).despawn_recursive();

    next_state.set(GameStates::Medium);
}

fn on_hard(
    mut event: EventReader<HardBtnPress>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut commands: Commands,
    levels_menu: Query<Entity, With<LevelsMenu>>,
) {
    if event.read().len() == 0 {
        return;
    }

    let levels_menu = levels_menu.single();
    commands.entity(levels_menu).despawn_recursive();

    next_state.set(GameStates::Hard);
}

fn on_expert(
    mut event: EventReader<ExpertBtnPress>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut commands: Commands,
    levels_menu: Query<Entity, With<LevelsMenu>>,
) {
    if event.read().len() == 0 {
        return;
    }

    let levels_menu = levels_menu.single();
    commands.entity(levels_menu).despawn_recursive();

    next_state.set(GameStates::Expert);
}

fn on_back(
    mut event: EventReader<BackBtnPress>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut commands: Commands,
    levels_menu: Query<Entity, With<LevelsMenu>>,
) {
    if event.read().len() == 0 {
        return;
    }

    let levels_menu = levels_menu.single();
    commands.entity(levels_menu).despawn_recursive();

    next_state.set(GameStates::MainMenu);
}
