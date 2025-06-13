use bevy::prelude::*;

use crate::{
    game::GameStates,
    items::upgrades::UpgradeLevel,
    shop::shop_items::{ItemType, ShopPosition},
};

pub struct ItemsUIPlugin;

// keeps id of UI window

#[derive(Component)]
pub struct HoverWindow;

#[derive(Component)]
pub struct HoverName;

#[derive(Component)]
pub struct HoverLevel;

#[derive(Component)]
pub struct HoverCost;

#[derive(Resource, Default)]
pub struct HoveredItem(pub Option<Entity>);

impl Plugin for ItemsUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HoveredItem>();
        app.add_systems(Startup, init_ui);
        app.add_systems(Update, make_hoverable);
        app.add_systems(Update, update_ui);
        app.add_systems(OnExit(GameStates::InGame), disable_hover_on_leave);
    }
}

fn update_ui(
    items: Query<(&ItemType, &GlobalTransform, Option<&UpgradeLevel>)>,
    mut hover_window: Single<&mut Node, With<HoverWindow>>,
    mut hover_name: Single<&mut Text, With<HoverName>>,
    mut hover_level: Single<&mut Text, (With<HoverLevel>, Without<HoverName>)>,
    mut hover_cost: Single<&mut Text, (With<HoverCost>, Without<HoverName>, Without<HoverLevel>)>,
    camera: Single<(&Camera, &GlobalTransform)>,
    hovered_item: Res<HoveredItem>,
) {
    let Some(item_id) = hovered_item.0 else {
        hover_name.0 = String::new();
        hover_level.0 = String::new();
        hover_cost.0 = String::new();

        return;
    };

    let Ok((item_type, item_transform, upgrade_level)) = items.get(item_id) else {
        return;
    };

    let pos = camera
        .0
        .world_to_viewport(camera.1, item_transform.translation())
        .expect("camera panik");

    hover_window.top = Val::Px(pos.y + 10.);
    hover_window.left = Val::Px(pos.x + 10.);

    hover_name.0 = item_type.name();

    hover_level.0 = match upgrade_level {
        None => "Fully Upgraded".to_string(),
        Some(x) => format!("Level {}", x.level + 1),
    };

    hover_cost.0 = match upgrade_level {
        None => "".to_string(),
        Some(x) => match x.next_price {
            None => "".to_string(),
            Some(p) => format!("Upgrade Cost: {p}"),
        },
    };
}

fn init_ui(mut commands: Commands) {
    let hover_window = commands
        .spawn((
            Name::new("HoverWindow"),
            Node {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            HoverWindow,
        ))
        .id();

    let hover_name = commands
        .spawn((
            Node { ..default() },
            Text::new("..."),
            TextFont::from_font_size(15.),
            HoverName,
        ))
        .id();

    let hover_level = commands
        .spawn((
            Node { ..default() },
            Text::new("..."),
            TextFont::from_font_size(15.),
            HoverLevel,
        ))
        .id();

    let hover_cost = commands
        .spawn((
            Node { ..default() },
            Text::new("..."),
            TextFont::from_font_size(15.),
            HoverCost,
        ))
        .id();

    commands
        .entity(hover_window)
        .add_child(hover_name)
        .add_child(hover_level)
        .add_child(hover_cost);
}

fn make_hoverable(
    items: Query<Entity, (Added<ItemType>, Without<ShopPosition>)>,
    mut commands: Commands,
) {
    for item in &items {
        commands
            .entity(item)
            .observe(on_hover_enter)
            .observe(on_hover_leave);
    }
}

fn disable_hover_on_leave(mut hovered_item: ResMut<HoveredItem>) {
    hovered_item.0 = None;
}

fn on_hover_enter(
    trigger: Trigger<Pointer<Over>>,
    mut hovered_item: ResMut<HoveredItem>,
    game_state: Res<State<GameStates>>,
) {
    if *game_state.get() != GameStates::InGame {
        return;
    }

    hovered_item.0 = Some(trigger.target);
}

fn on_hover_leave(trigger: Trigger<Pointer<Out>>, mut hovered_item: ResMut<HoveredItem>) {
    if let Some(item) = hovered_item.0 {
        if item == trigger.target {
            hovered_item.0 = None;
        }
    }
}
