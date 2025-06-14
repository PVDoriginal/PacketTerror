use bevy::prelude::*;
use ui::{DefeatUIPlugin, FiveRonBtnPress};

use crate::game::GameStates;

pub mod ui;

#[derive(Component)]
pub struct DefeatScreen;

pub struct DefeatScreenPlugin;

impl Plugin for DefeatScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefeatUIPlugin);
        app.add_systems(
            Update,
            (on_return).run_if(in_state(GameStates::DefeatScreen)),
        );
    }
}

//system to to the main menu
fn on_return(
    mut event: EventReader<FiveRonBtnPress>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut commands: Commands,
    defeat_screen: Query<Entity, With<DefeatScreen>>,
) {
    if event.read().len() == 0 {
        return;
    }
    let defeat_screen = defeat_screen.single();
    commands.entity(defeat_screen).despawn_recursive(); //remove the elements of the defeat screen

    next_state.set(GameStates::MainMenu); //return to main menu
}
