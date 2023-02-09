use crate::GameState;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(hide_cursor))
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(unhide_cursor));
    }
}

fn hide_cursor(mut window: ResMut<Windows>) {
    let window = window.get_primary_mut().unwrap();

    window.set_cursor_grab_mode(CursorGrabMode::Confined);
    window.set_cursor_visibility(false);
    println!("ahoj");
}

fn unhide_cursor(mut window: ResMut<Windows>) {
    let window = window.get_primary_mut().unwrap();

    window.set_cursor_grab_mode(CursorGrabMode::None);
    window.set_cursor_visibility(true);
}
