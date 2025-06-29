use std::{cmp, fs::File, io::Write};

use crate::{
    camera::SPRITE_SIZE,
    game::{GameLevels, GameStates, HIGHEST_LEVEL_PATH, HighestLevel, InGame},
    grid::Grid,
    levels::{Level, WaveManager, advance_level, get_level},
};
use bevy::prelude::*;

use super::{
    cables::{Cable, get_adj_cables},
    packets::{EnemyPacket, Packet},
};

#[derive(Component)]
#[require(InGame)]
pub struct EnemyPC;
pub struct EnemyPCPlugin;

impl Plugin for EnemyPCPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WaveManager::default());
        app.add_systems(OnEnter(GameStates::InGame), load_level);
        app.add_systems(Update, create_packets.run_if(in_state(GameStates::InGame)));
    }
}

fn load_level(mut wave_manager: ResMut<WaveManager>, state: Res<State<GameLevels>>) {
    *wave_manager = WaveManager::default();
    let lvl: Level = get_level((**state).clone());
    wave_manager.level = Some(lvl.clone());
    wave_manager.timer = (&lvl).get_timer(0, 0);
}

//sends packets to your pc
fn create_packets(
    packet_senders: Query<&Transform, With<EnemyPC>>,
    cables: Query<&Cable>,
    grid: ResMut<Grid>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut wave_manager: ResMut<WaveManager>,
    enemy_packets: Query<&EnemyPacket>,
    state: Res<State<GameLevels>>,
    mut highest: ResMut<HighestLevel>,
    mut next_state: ResMut<NextState<GameStates>>,
) {
    let all_enemies_killed = enemy_packets.is_empty();

    let Some(packet_type) = advance_level(
        &mut wave_manager,
        &time,
        all_enemies_killed,
        *state == GameLevels::Sandbox,
    ) else {
        if all_enemies_killed && !wave_manager.valid() {
            let val: u8 = u8::from(**state);
            highest.highest = cmp::max(val, highest.highest.into()).into();

            let Ok(mut file) = File::create(HIGHEST_LEVEL_PATH) else {
                return;
            };
            file.write(&[Into::<u8>::into(highest.highest)]).ok();

            next_state.set(GameStates::VictoryScreen);
        }

        return;
    };

    for packet_sender in &packet_senders {
        let cables = get_adj_cables(packet_sender.translation.truncate(), &cables, &grid);

        for (cable_pos, adj_space) in cables {
            commands.spawn((
                EnemyPacket,
                Packet::new(adj_space, packet_type),
                Sprite::from_image(asset_server.load("enemy_packet.png")),
                Transform::from_translation(
                    (cable_pos - adj_space * SPRITE_SIZE / 2.05).extend(2.),
                ),
                Name::from("Enemy packet"),
            ));
        }
    }
}
