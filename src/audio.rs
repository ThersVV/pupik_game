use bevy::prelude::*;
use bevy_kira_audio::Audio;
use bevy_kira_audio::*;
use std::time::Duration;
///[Plugin] taking care of audio related functionalities.
pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, play_background_music);
    }
}
///Turns on background music on repeat.
/// # Arguments
/// * `asset_server` - [AssetServer], used to load the music in.
/// * `audio` - [Audio].
fn play_background_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio
        .play(asset_server.load("bckgrnd_msc.wav")) //Music by Vojtěch Klhůfek
        .fade_in(AudioTween::new(
            Duration::from_secs_f32(0.5),
            AudioEasing::OutPowi(2),
        ))
        .with_volume(0.3)
        .looped();
}
