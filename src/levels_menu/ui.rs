use bevy::color::palettes::basic::*;
use bevy::prelude::*;

use crate::game::GameStates;

use super::LevelsMenu;

//colors for each button
const EASY_BUTTON: Color = Color::srgb(0., 0.25, 0.);
const MEDIUM_BUTTON: Color = Color::srgb(0.25, 0.25, 0.);
const HARD_BUTTON: Color = Color::srgb(0.25, 0., 0.);
const EXPERT_BUTTON: Color = Color::srgb(0., 0., 0.25);
const BACK_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.35, 0.35);

#[derive(Event)]
pub struct EasyBtnPress;

#[derive(Event)]
pub struct MediumBtnPress;

#[derive(Event)]
pub struct HardBtnPress;

#[derive(Event)]
pub struct ExpertBtnPress;

#[derive(Event)]
pub struct BackBtnPress;

#[derive(Component)]
pub enum ButtonType {
    Easy,
    Medium,
    Hard,
    Expert,
    Back,
}

pub struct LevUIPlugin;

impl Plugin for LevUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EasyBtnPress>();
        app.add_event::<MediumBtnPress>();
        app.add_event::<HardBtnPress>();
        app.add_event::<ExpertBtnPress>();
        app.add_event::<BackBtnPress>();
        app.add_systems(OnEnter(GameStates::LevelsMenu), setup);
        app.add_systems(
            Update,
            button_system.run_if(in_state(GameStates::LevelsMenu)),
        );
    }
}

//manage button interactions
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
    mut easy: EventWriter<EasyBtnPress>,
    mut medium: EventWriter<MediumBtnPress>,
    mut hard: EventWriter<HardBtnPress>,
    mut expert: EventWriter<ExpertBtnPress>,
    mut back: EventWriter<BackBtnPress>,
) {
    for (interaction, btn_type, mut color, mut border_color, _children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();

                match btn_type {
                    ButtonType::Easy => {
                        easy.send(EasyBtnPress);
                    }
                    ButtonType::Medium => {
                        medium.send(MediumBtnPress);
                    }
                    ButtonType::Hard => {
                        hard.send(HardBtnPress);
                    }
                    ButtonType::Expert => {
                        expert.send(ExpertBtnPress);
                    }
                    ButtonType::Back => {
                        back.send(BackBtnPress);
                    }
                }
            }
            Interaction::Hovered => {
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                border_color.0 = Color::BLACK;
            }
        }
    }
}

//create the UI
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Name::new("levels menu"),
            LevelsMenu,
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
                Text::new("LEVEL SELECT"),
                TextFont {
                    font: asset_server.load("fonts/courbd.ttf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.0, 0.0)),
            ));
            //this below is the easy button
            parent
                .spawn((
                    Button,
                    ButtonType::Easy,
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
                    BackgroundColor(EASY_BUTTON),
                ))
                .with_child((
                    Text::new("EASY"),
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
                    ButtonType::Medium,
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
                    BackgroundColor(MEDIUM_BUTTON),
                ))
                .with_child((
                    Text::new("MEDIUM"),
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
                    ButtonType::Hard,
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
                    BackgroundColor(HARD_BUTTON),
                ))
                .with_child((
                    Text::new("HARD"),
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
                    ButtonType::Expert,
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
                    BackgroundColor(EXPERT_BUTTON),
                ))
                .with_child((
                    Text::new("EXPERT"),
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
                    ButtonType::Back,
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
                    BackgroundColor(BACK_BUTTON),
                ))
                .with_child((
                    Text::new("BACK"),
                    TextFont {
                        font: asset_server.load("fonts/courbd.ttf"),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });
}
