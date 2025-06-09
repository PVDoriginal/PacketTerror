use bevy::{math::vec3, prelude::*};

use crate::grid::GRID_M;
use crate::grid::GRID_N;

pub const SCALE: f32 = 0.5;
pub const SPRITE_SIZE: f32 = 21.;

#[derive(Resource, Default)]
pub struct Screen {
    pub rect: Rect,
}

#[derive(Component)]
pub struct Shake {
    strength: f32,
    timer: Timer,
    initial_pos: Vec3,
}
impl Shake {
    pub fn new(strength: f32, duration: f32, initial_pos: Vec3) -> Self {
        Self {
            strength,
            timer: Timer::from_seconds(duration, TimerMode::Once),
            initial_pos,
        }
    }
}
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Screen>();

        app.add_systems(Startup, init_camera);
        app.add_systems(Update, update_screen);
        app.add_systems(Update, shake);
    }
}

pub fn init_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scale: SCALE,
            ..OrthographicProjection::default_2d()
        },
        Msaa::Off,
        Transform::from_translation(vec3(
            GRID_N as f32 / 2. * SPRITE_SIZE - SPRITE_SIZE,
            GRID_M as f32 / 2. * SPRITE_SIZE - 34., // subtracted value is arbitrary so it looks good
            10.,
        )),
    ));
}

pub fn update_screen(
    mut screen: ResMut<Screen>,
    windows: Query<&Window>,
    cameras: Query<(&Transform, &OrthographicProjection), With<Camera2d>>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };

    let Ok((camera, projection)) = cameras.get_single() else {
        return;
    };

    let center = camera.translation.truncate();
    let half_width = (window.width() / 2.0) * projection.scale;
    let half_height = (window.height() / 2.0) * projection.scale;

    screen.rect = Rect::new(
        center.x - half_width,
        center.y + half_height,
        center.x + half_width,
        center.y - half_height,
    );
}

fn shake(
    mut shakable: Query<(Entity, &mut Shake, &mut Transform)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut shake, mut transl) in shakable.iter_mut() {
        if shake.timer.tick(time.delta()).just_finished() {
            transl.translation = shake.initial_pos;
            commands.entity(entity).remove::<Shake>();
            continue;
        }

        transl.translation += vec3(
            (rand::random::<f32>() - 0.5) * 100. * time.delta_secs() * shake.strength,
            (rand::random::<f32>() - 0.5) * 100. * time.delta_secs() * shake.strength,
            0.,
        );
    }
}
