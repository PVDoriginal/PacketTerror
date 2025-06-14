use bevy::color::palettes::basic::*;
use bevy::prelude::*;

use crate::game::GameStates;

use super::MainMenu;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.35, 0.35);

#[derive(Event)]
pub struct PlayBtnPress;

#[derive(Event)]
pub struct LevelsBtnPress;

#[derive(Event)]
pub struct QuitBtnPress;

#[derive(Component)]
pub enum ButtonType {
    Play,
    Levels,
    Quit,
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayBtnPress>();
        app.add_event::<LevelsBtnPress>();
        app.add_event::<QuitBtnPress>();
        app.add_systems(OnEnter(GameStates::MainMenu), setup);
        app.add_systems(Update, button_system.run_if(in_state(GameStates::MainMenu)));
    }
}

//manages interactions
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
    mut play: EventWriter<PlayBtnPress>,
    mut levels: EventWriter<LevelsBtnPress>,
    mut quit: EventWriter<QuitBtnPress>,
) {
    for (interaction, btn_type, mut color, mut border_color, _children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();

                match btn_type {
                    ButtonType::Play => {
                        play.send(PlayBtnPress);
                    }
                    ButtonType::Levels => {
                        levels.send(LevelsBtnPress);
                    }
                    ButtonType::Quit => {
                        quit.send(QuitBtnPress);
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

//creates the UI
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Name::new("main menu"),
            MainMenu,
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
                Text::new("PACKET TERROR"),
                TextFont {
                    font: asset_server.load("fonts/courbd.ttf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.0, 0.0)),
            ));
            parent
                .spawn((
                    Button,
                    ButtonType::Play,
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
                    Text::new("Sandbox"),
                    TextFont {
                        font: asset_server.load("fonts/courbd.ttf"),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
            parent
                .spawn((
                    Button,
                    ButtonType::Levels,
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
                    Text::new("Levels"),
                    TextFont {
                        font: asset_server.load("fonts/courbd.ttf"),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
            parent
                .spawn((
                    Button,
                    ButtonType::Quit,
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
                    Text::new("Quit"),
                    TextFont {
                        font: asset_server.load("fonts/courbd.ttf"),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });
}
