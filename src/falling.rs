use crate::{speed::Speed, GameState};
use bevy::prelude::*;

/// [Plugin] taking care of all movement and despawning of falling [entities](Entity). Does *not* handle [Plane], because its
/// despawn condition has to be different.
pub struct FallPlugin;

///Labels [entities](Entity) which fall and despawn once not visible.
#[derive(Component, Deref, DerefMut)]
pub struct FallTimer(pub Timer);

impl Plugin for FallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Game).with_system(ingame_despawn))
            .add_system_set(SystemSet::on_exit(GameState::EndScreen).with_system(endscreen_despawn))
            .add_system(movement);
    }
}

///Despawns [entities](Entity) with [FallTimer] once not visible.
/// # Arguments
/// * `commands` - [Commands].
/// * `query` - [Query] for [FallTimer].
/// * `speed` - [Speed]. Despawns happen sooner with higher speed.
/// * `time` - [Time].
fn ingame_despawn(
    mut commands: Commands,
    mut query: Query<(Entity, &mut FallTimer), With<FallTimer>>,
    speed: Res<Speed>,
    time: Res<Time>,
) {
    let speed = speed.speed;
    for (entity, mut timer) in &mut query {
        timer.tick(time.delta().mul_f32(speed));
        if timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

///Despawns [entities](Entity) with [FallTimer] on exit from [GameState::EndScreen].
/// # Arguments
/// * `commands` - [Commands].
/// * `query` - [Query] for [FallTimer].
fn endscreen_despawn(mut commands: Commands, query: Query<Entity, With<FallTimer>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
///Moves [entities](Entity) with [FallTimer] down.
/// # Arguments
/// * `query` - [Query] for [FallTimer].
/// * `speed` - [Speed].
/// * `time` - [Time].
fn movement(mut query: Query<&mut Transform, With<FallTimer>>, time: Res<Time>, speed: Res<Speed>) {
    for mut transform in query.iter_mut() {
        transform.translation.y -= 200. * speed.speed * time.delta_seconds();
    }
}
