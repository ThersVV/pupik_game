use crate::{speed::Speed, GameState, PrimaryWindow};
use bevy::prelude::*;

/// [Plugin] taking care of all movement and despawning of falling [entities](Entity). Does *not* handle [Plane], because its
/// despawn condition has to be different.
pub struct FallPlugin;

///Labels [entities](Entity) which fall and despawn once not visible.
#[derive(Component, Deref, DerefMut)]
pub struct FallTimer(pub Timer);

impl Plugin for FallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::EndScreen), endscreen_despawn)
            .add_systems(Update, (movement, ingame_despawn));
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
    query: Query<(Entity, &Transform), With<FallTimer>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    let w_height = q_windows.single().height();
    for (entity, trans) in query.iter() {
        if trans.translation.y < (w_height / -2.) - 200. {
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
