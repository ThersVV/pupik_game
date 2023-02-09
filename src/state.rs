use crate::GameState;
use bevy::prelude::*;
pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app/* .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(exit_main_menu)) */
            .add_system_set(
                SystemSet::on_update(GameState::EndScreen).with_system(exit_end_screen),
            );
    }
}

/* fn exit_main_menu(mouse: Res<Input<MouseButton>>, mut state: ResMut<State<GameState>>) {
    if mouse.just_pressed(MouseButton::Left) {
        state
            .set(GameState::Game)
            .expect("Unexpected state set error.");
    }
}
 */
fn exit_end_screen(mouse: Res<Input<MouseButton>>, mut state: ResMut<State<GameState>>) {
    if mouse.just_pressed(MouseButton::Right) {
        state
            .set(GameState::MainMenu)
            .expect("Unexpected state set error.");
    }
}
