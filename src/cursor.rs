use crate::GameState;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;

///[Plugin] taking care of cursor related functionalities. This plugin contains
/// * [hide_cursor]
/// * [unhide_cursor]
pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(hide_cursor))
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(unhide_cursor));
    }
}
/// Hides the cursor and locks it inside the game window. It is run on enter in the [GameState::Game].
/// # Arguments
/// * `window` - [Resource] containing [Windows].
fn hide_cursor(mut window: ResMut<Windows>) {
    let window = window.get_primary_mut().unwrap();

    window.set_cursor_grab_mode(CursorGrabMode::Confined);
    window.set_cursor_visibility(false);
}

/// Unhides the cursor and unlocks it from the game window. It is run on exit in the [GameState::Game].
/// # Arguments
/// * `window` - [Resource] containing [Windows].
fn unhide_cursor(mut window: ResMut<Windows>) {
    let window = window.get_primary_mut().unwrap();

    window.set_cursor_grab_mode(CursorGrabMode::None);
    window.set_cursor_visibility(true);
}
