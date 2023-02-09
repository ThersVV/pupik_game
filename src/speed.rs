use crate::{GameState, Settings, Speed};
use bevy::prelude::*;
pub struct SpeedPlugin;

impl Plugin for SpeedPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_tachometer)
            .add_system_set(SystemSet::on_update(GameState::Game).with_system(scaling));
    }
}

fn scaling(mut query: Query<&mut Speed, With<Speed>>, time: Res<Time>) {
    for mut speed in query.iter_mut() {
        speed.num += 0.01 * time.delta_seconds();
    }
}

fn spawn_tachometer(mut commands: Commands, settings: Res<Settings>) {
    let switch = commands
        .spawn(Speed {
            num: settings.startup_speed,
        })
        .id();
    commands.entity(switch);
}
