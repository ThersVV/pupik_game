use crate::{FallTimer, Speed};
use bevy::prelude::*;
pub struct FallPlugin;

impl Plugin for FallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(despawn_all).add_system(movement);
    }
}

fn despawn_all(
    mut commands: Commands,
    mut query: Query<(Entity, &mut FallTimer), With<FallTimer>>,
    time: Res<Time>,
) {
    for (entity, mut timer) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            commands.entity(entity).despawn();
        }
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
