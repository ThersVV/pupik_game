use crate::GameState;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy::window::PrimaryWindow;

///[Plugin] taking care of cursor related functionalities. This plugin contains
/// * [hide_cursor]
/// * [unhide_cursor]
pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), hide_cursor)
            .add_systems(OnExit(GameState::Game), unhide_cursor);
    }
}
/// Hides the cursor and locks it inside the game window. It is run on enter in the [GameState::Game].
/// # Arguments
/// * `window` - [Resource] containing [Windows].
fn hide_cursor(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(window) = &mut window.get_single_mut() {
        window.cursor.grab_mode = CursorGrabMode::Confined;
        window.cursor.visible = false;
    }
}

/// Unhides the cursor and unlocks it from the game window. It is run on exit in the [GameState::Game].
/// # Arguments
/// * `window` - [Resource] containing [Windows].
fn unhide_cursor(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(window) = &mut window.get_single_mut() {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}
