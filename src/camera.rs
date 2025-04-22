use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Screen {
    pub rect: Rect,
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_camera);

        app.init_resource::<Screen>();
        app.add_systems(Update, update_screen);
    }
}

pub fn init_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scale: 0.2,
            ..OrthographicProjection::default_2d()
        },
        Msaa::Off,
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
