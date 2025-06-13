use bevy::color::palettes::basic::*;
use bevy::prelude::*;

use crate::game::GameStates;

use super::VictoryScreen;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.35, 0.35);

#[derive(Event)]
pub struct WinBtnPress;

#[derive(Component)]
pub enum ButtonType {
    Win,
}

pub struct VictoryUIPlugin;

impl Plugin for VictoryUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WinBtnPress>();
        app.add_systems(OnEnter(GameStates::VictoryScreen), setup);
        app.add_systems(
            Update,
            button_system.run_if(in_state(GameStates::VictoryScreen)),
        );
    }
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &ButtonType,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        Changed<Interaction>,
    >,
    mut win: EventWriter<WinBtnPress>,
) {
    for (interaction, btn_type, mut color, mut border_color, _children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();

                match btn_type {
                    ButtonType::Win => {
                        win.send(WinBtnPress);
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Name::new("victory screen"),
            VictoryScreen,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("VICTORY!"),
                TextFont {
                    font: asset_server.load("fonts/courbd.ttf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 1.0, 0.0)),
            ));
            parent
                .spawn((
                    Button,
                    ButtonType::Win,
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_child((
                    Text::new("Back to Menu"),
                    TextFont {
                        font: asset_server.load("fonts/courbd.ttf"),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });
}
