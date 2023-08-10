use crate::{text::Score, GameState};
use bevy::prelude::*;
use bevy_pkv::PkvStore;
///[Plugin] taking care of audio related functionalities.
pub struct HighScorePlugin;

impl Plugin for HighScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Game), update_highscore);
    }
}
///Turns on background music on repeat.
/// # Arguments
/// * `asset_server` - [AssetServer], used to load the music in.
/// * `audio` - [Audio].
fn update_highscore(mut pkv: ResMut<PkvStore>, score: Res<Score>) {
    let score = score.score as i32;
    if let Ok(highscore) = pkv.get::<String>("highscore") {
        if highscore.parse::<i32>().unwrap() < score {
            pkv.set("highscore", &score.to_string())
                .expect("failed to store score");
        }
    } else {
        pkv.set("highscore", &score.to_string())
            .expect("failed to store score");
    }
}
