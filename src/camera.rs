use bevy::{math::vec3, prelude::*};

use crate::grid::GRID_M;
use crate::grid::GRID_N;

pub const SCALE: f32 = 0.5;
pub const SPRITE_SIZE: f32 = 21.;

#[derive(Resource, Default)]
pub struct Screen {
    pub rect: Rect,
}
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Screen>();

        app.add_systems(Startup, init_camera);
        app.add_systems(Update, update_screen);
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

    //right in the middle
    screen.rect = Rect::new(
        center.x - half_width,
        center.y + half_height,
        center.x + half_width,
        center.y - half_height,
    );
}
