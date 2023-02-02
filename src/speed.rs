use crate::Speed;
use bevy::prelude::*;
pub struct SpeedPlugin;

impl Plugin for SpeedPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_tachometer).add_system(scaling);
    }
}

fn scaling(mut query: Query<&mut Speed, With<Speed>>, time: Res<Time>) {
    for mut speed in query.iter_mut() {
        speed.num += 0.015 * time.delta_seconds();
    }
}

fn spawn_tachometer(mut commands: Commands) {
    let switch = commands.spawn(Speed { num: 0.7 }).id();
    commands.entity(switch);
}
