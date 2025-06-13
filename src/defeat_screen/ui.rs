use bevy::color::palettes::basic::*;
use bevy::prelude::*;

use crate::game::GameStates;

use super::DefeatScreen;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.35, 0.35);

#[derive(Event)]
pub struct FiveRonBtnPress;

#[derive(Component)]
pub enum ButtonType {
    FiveRon,
}

pub struct DefeatUIPlugin;

impl Plugin for DefeatUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FiveRonBtnPress>();
        app.add_systems(OnEnter(GameStates::DefeatScreen), setup);
        app.add_systems(
            Update,
            button_system.run_if(in_state(GameStates::DefeatScreen)),
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
    mut fiveron: EventWriter<FiveRonBtnPress>,
) {
    for (interaction, btn_type, mut color, mut border_color, _children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();

                match btn_type {
                    ButtonType::FiveRon => {
                        fiveron.send(FiveRonBtnPress);
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
            Name::new("defeat screen"),
            DefeatScreen,
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
                Text::new("YOU DESTROYED THE EQUIPMENT, MR STUDENT!"),
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
                    ButtonType::FiveRon,
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
                    Text::new("Pay 5RON"),
                    TextFont {
                        font: asset_server.load("fonts/courbd.ttf"),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });
}
