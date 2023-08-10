use crate::{GameState, Settings};
use bevy::prelude::*;

///[Plugin] taking care of functionalities corelating with [Speed].
pub struct SpeedPlugin;

///A speed multiplier that increases with time.
/// # Fields
/// `speed` - Current speed multiplier.
#[derive(Resource)]
pub struct Speed {
    pub speed: f32,
}

impl Plugin for SpeedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tachometer)
            .add_systems(Update, scaling.run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::EndScreen), reset_speed);
    }
}

///Scales [Speed] with time.
/// # Arguments
/// * `speed` - [Speed].
/// * `time` - [Time].
/// * `settings` - [Settings], used to access the `speed_scaling` field.
fn scaling(mut speed: ResMut<Speed>, time: ResMut<Time>, settings: Res<Settings>) {
    speed.speed += settings.speed_scaling * time.delta_seconds();
}

///Spawns [Speed].
/// # Arguments
/// * `commands` - [Commands]
/// * `settings` - [Settings], used to access the `startup_speed` field.
fn spawn_tachometer(mut commands: Commands, settings: Res<Settings>) {
    commands.insert_resource(Speed {
        speed: settings.startup_speed,
    });
}
///Resets [Speed] on exit from [Game::Endscreen].
/// # Arguments
/// * `commands` - [Commands].
/// * `settings` - [Settings], used to access the `startup_speed` field.
fn reset_speed(settings: Res<Settings>, mut speed: ResMut<Speed>) {
    speed.speed = settings.startup_speed;
}
