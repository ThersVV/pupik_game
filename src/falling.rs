use crate::{FallTimer, GameState, Speed};
use bevy::prelude::*;
pub struct FallPlugin;

impl Plugin for FallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Game).with_system(ingame_despawn))
            .add_system_set(SystemSet::on_exit(GameState::EndScreen).with_system(endscreen_despawn))
            .add_system(movement);
    }
}

fn ingame_despawn(
    mut commands: Commands,
    mut query: Query<(Entity, &mut FallTimer), With<FallTimer>>,
    speed: Query<&Speed, With<Speed>>,
    time: Res<Time>,
) {
    let speed = speed.single().num;
    for (entity, mut timer) in &mut query {
        timer.tick(time.delta().mul_f32(speed));
        if timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn endscreen_despawn(mut commands: Commands, query: Query<Entity, With<FallTimer>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn movement(
    mut query: Query<&mut Transform, With<FallTimer>>,
    time: Res<Time>,
    speed: Query<&Speed, With<Speed>>,
) {
    for mut transform_hole in query.iter_mut() {
        let speed = speed.single();
        transform_hole.translation.y -= 200. * speed.num * time.delta_seconds();
    }
}
