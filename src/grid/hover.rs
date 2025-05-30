use bevy::color::palettes::basic::*;
use bevy::prelude::*;

use crate::game::GameStates;

use super::MainMenu;

#[derive(Event)]
pub struct RouterBtn;

#[derive(Event)]
pub struct SwitchBtn;

#[derive(Event)]
pub struct ServerBtn;

#[derive(Event)]
pub struct CableBtn;

#[derive(Component)]
pub enum ButtonType {
    Router,
    Switch,
    Server,
    Cable,
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RouterHover>();
        app.add_event::<SwitchHover>();
        app.add_event::<ServerHover>();
        app.add_event::<CableHover>();
        app.add_systems(OnEnter(GameStates::Game), setup);
        app.add_systems(Update, hover_system.run_if(in_state(GameStates::Game)));
    }
}

fn hover_system(
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
    mut rt_hover: EventWriter<RouterHover>,
    mut sw_hover: EventWriter<SwitchHover>,
    mut sv_hover: EventWriter<ServerHover>,
    mut cb_hover: EventWriter<CableHover>,
) {
    for (interaction, btn_type, mut color, mut border_color, _children) in &mut interaction_query {
        match *interaction {
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

/*
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((Name::new("main menu"), MainMenu, Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        }))
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
                        width: Val::Px(150.0),
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
                    Text::new("Play"),
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
                        width: Val::Px(150.0),
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
*/
