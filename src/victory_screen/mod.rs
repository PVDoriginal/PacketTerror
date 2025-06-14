use bevy::prelude::*;
use ui::{VictoryUIPlugin, WinBtnPress};

use crate::game::GameStates;

pub mod ui;

#[derive(Component)]
pub struct VictoryScreen;

pub struct VictoryScreenPlugin;

impl Plugin for VictoryScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VictoryUIPlugin);
        app.add_systems(
            Update,
            (on_return).run_if(in_state(GameStates::VictoryScreen)),
        );
    }
}

//back to main menu
fn on_return(
    mut event: EventReader<WinBtnPress>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut commands: Commands,
    victory_screen: Query<Entity, With<VictoryScreen>>,
) {
    if event.read().len() == 0 {
        return;
    }
    let victory_screen = victory_screen.single();
    commands.entity(victory_screen).despawn_recursive();

    next_state.set(GameStates::MainMenu);
}
