use bevy::{math::vec3, prelude::*};
const SHAKE_MULT: f32 = 80.;

#[derive(Component)]
pub struct Shake {
    strength: f32,
    timer: Timer,
    initial_pos: Vec3,
}

//decide how strong and how long will the shake be
impl Shake {
    pub fn new(strength: f32, duration: f32, initial_pos: Vec3) -> Self {
        Self {
            strength,
            timer: Timer::from_seconds(duration, TimerMode::Once),
            initial_pos,
        }
    }
}

//shaking that grid
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
            (rand::random::<f32>() - 0.5) * SHAKE_MULT * time.delta_secs() * shake.strength,
            (rand::random::<f32>() - 0.5) * SHAKE_MULT * time.delta_secs() * shake.strength,
            0.,
        );
    }
}

pub struct ShakePlugin;

impl Plugin for ShakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, shake);
    }
}
