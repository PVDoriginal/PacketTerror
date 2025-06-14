use bevy::prelude::*;
use std::time::Duration;

use crate::{game::GameLevels, items::packets::PacketType};

#[derive(Resource)]
pub struct WaveManager {
    pub timer: Timer,
    pub level: Option<Level>,
    i: usize,
    j: usize,
}
impl WaveManager {
    pub fn valid(&self) -> bool {
        if self.level.is_none() {
            return false;
        }
        return self.level.as_ref().unwrap().valid(self.i, self.j);
    }
    pub fn default() -> WaveManager {
        WaveManager {
            timer: Timer::new(Duration::from_secs_f32(0.), TimerMode::Once),
            level: None,
            i: 0,
            j: 0,
        }
    }
    pub fn get_index(&self) -> (usize, usize) {
        (self.i, self.j)
    }
    pub fn get_sandbox_timer() -> Timer {
        return Timer::new(Duration::from_secs_f32(3.), TimerMode::Once);
    }
}
#[derive(Clone)]
pub struct Wave {
    wave: Vec<(PacketType, f32)>,
}
#[derive(Clone)]
pub struct Level {
    waves: Vec<Wave>,
}
impl Level {
    pub fn valid(&self, i: usize, j: usize) -> bool {
        return i < self.waves.len() && j < self.waves[i].wave.len();
    }
    pub fn get_timer(&self, i: usize, j: usize) -> Timer {
        if !self.valid(i, j) {
            return Timer::new(Duration::from_secs_f32(0.), TimerMode::Once);
        }

        // info!("{}", self.waves[i].wave[j].1);
        return Timer::new(
            Duration::from_secs_f32(self.waves[i].wave[j].1),
            TimerMode::Once,
        );
    }
}

pub fn advance_level(
    wave_manager: &mut ResMut<WaveManager>,
    time: &Res<Time>,
    can_start_next_wave: bool,
    in_sandbox: bool,
) -> Option<PacketType> {
    if wave_manager.level.is_none() && !in_sandbox {
        return None;
    };

    if !in_sandbox && wave_manager.i == 0 && wave_manager.j == 0 {
        wave_manager.j += 1;

        wave_manager.timer = wave_manager
            .level
            .as_ref()
            .unwrap()
            .get_timer(wave_manager.i, wave_manager.j);

        return Some(wave_manager.level.as_ref().unwrap().waves[0].wave[0].0);
    }

    wave_manager.timer.tick(time.delta());

    if wave_manager.timer.finished() && in_sandbox {
        wave_manager.timer = WaveManager::get_sandbox_timer();
        wave_manager.timer = WaveManager::get_sandbox_timer();
        return Some(PacketType::Basic);
    }

    if wave_manager.timer.finished() {
        wave_manager.j += 1;

        if wave_manager.j
            >= wave_manager.level.as_ref().unwrap().waves[wave_manager.i]
                .wave
                .len()
        {
            if can_start_next_wave {
                wave_manager.i += 1;
                wave_manager.j = 0;
            } else {
                return None;
            }
        }

        if wave_manager.i >= wave_manager.level.as_ref().unwrap().waves.len() {
            **wave_manager = WaveManager::default();
            return None;
        }

        wave_manager.timer = wave_manager
            .level
            .as_ref()
            .unwrap()
            .get_timer(wave_manager.i, wave_manager.j);

        return Some(
            wave_manager.level.as_ref().unwrap().waves[wave_manager.i].wave[wave_manager.j].0,
        );
    } else {
        return None;
    }
}

pub fn get_level(level: GameLevels) -> Level {
    fn create_wave(packets: Vec<(PacketType, f32)>) -> Wave {
        Wave { wave: packets }
    }

    //create the waves for each level
    return match level {
        GameLevels::Sandbox => Level {
            waves: vec![create_wave(vec![(PacketType::Basic, 1.0)])],
        },
        GameLevels::Easy => Level {
            waves: vec![
                create_wave(vec![
                    (PacketType::Basic, 5.0),
                    (PacketType::Basic, 5.0),
                    (PacketType::Basic, 5.0),
                    (PacketType::Basic, 5.0),
                    (PacketType::Basic, 5.0),
                ]),
                create_wave(vec![
                    (PacketType::Basic, 3.0),
                    (PacketType::Basic, 3.0),
                    (PacketType::Basic, 3.0),
                    (PacketType::Basic, 3.0),
                    (PacketType::Basic, 3.0),
                ]),
            ],
        },
        GameLevels::Medium => Level {
            waves: vec![
                create_wave(vec![
                    (PacketType::Basic, 1.0),
                    (PacketType::Basic, 1.0),
                    (PacketType::Basic, 1.0),
                    (PacketType::Basic, 1.0),
                    (PacketType::Basic, 1.0),
                ]),
                create_wave(vec![
                    (PacketType::Basic, 0.8),
                    (PacketType::Basic, 0.8),
                    (PacketType::Basic, 0.8),
                    (PacketType::Basic, 0.8),
                    (PacketType::Basic, 0.8),
                ]),
                create_wave(vec![
                    (PacketType::Mid, 0.6),
                    (PacketType::Mid, 0.6),
                    (PacketType::Basic, 0.6),
                    (PacketType::Basic, 0.6),
                    (PacketType::Basic, 0.6),
                ]),
            ],
        },
        GameLevels::Hard => Level {
            waves: vec![
                create_wave(vec![
                    (PacketType::Basic, 1.0),
                    (PacketType::Basic, 1.0),
                    (PacketType::Basic, 1.0),
                    (PacketType::Basic, 1.0),
                    (PacketType::Basic, 1.0),
                ]),
                create_wave(vec![
                    (PacketType::Mid, 0.8),
                    (PacketType::Mid, 0.8),
                    (PacketType::Mid, 0.8),
                    (PacketType::Basic, 0.8),
                    (PacketType::Basic, 0.8),
                ]),
                create_wave(vec![
                    (PacketType::Mid, 0.6),
                    (PacketType::Mid, 0.6),
                    (PacketType::Basic, 0.6),
                    (PacketType::Basic, 0.6),
                    (PacketType::Basic, 0.6),
                ]),
                create_wave(vec![
                    (PacketType::Advanced, 0.5),
                    (PacketType::Advanced, 0.5),
                    (PacketType::Mid, 0.5),
                    (PacketType::Mid, 0.5),
                    (PacketType::Basic, 0.5),
                ]),
            ],
        },
        GameLevels::Expert => Level {
            waves: vec![
                create_wave(vec![
                    (PacketType::Basic, 1.0),
                    (PacketType::Basic, 1.0),
                    (PacketType::Basic, 1.0),
                    (PacketType::Basic, 1.0),
                    (PacketType::Basic, 1.0),
                ]),
                create_wave(vec![
                    (PacketType::Mid, 0.8),
                    (PacketType::Mid, 0.8),
                    (PacketType::Mid, 0.8),
                    (PacketType::Mid, 0.8),
                    (PacketType::Basic, 0.8),
                ]),
                create_wave(vec![
                    (PacketType::Mid, 0.6),
                    (PacketType::Mid, 0.6),
                    (PacketType::Basic, 0.6),
                    (PacketType::Basic, 0.6),
                    (PacketType::Basic, 0.6),
                ]),
                create_wave(vec![
                    (PacketType::Advanced, 0.5),
                    (PacketType::Advanced, 0.5),
                    (PacketType::Mid, 0.5),
                    (PacketType::Mid, 0.5),
                    (PacketType::Basic, 0.5),
                    (PacketType::Advanced, 0.5),
                ]),
            ],
        },
    };
}
