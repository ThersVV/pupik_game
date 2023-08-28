use crate::{
    collisions::Damaging,
    player::{point_distance, Hidden, Player},
    GameState, RainbowSheet,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
/// [Plugin] taking care of the [Rainbow] behaviour
pub struct RainbowPlugin;

///Labels an [Entity] that is either [Homing] or [Trail].
#[derive(Component)]
struct Rainbow;

///Labels an [Entity] that homes in on [Player] and spawns [Trail]s.
#[derive(Component)]
struct Homing;

///Labels an [Entity] that spawns behind [Homing] and acts as a trail.
///Contains a timer used for despawning
#[derive(Component, Deref, DerefMut)]
struct TrailTimer(Timer);

impl Plugin for RainbowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (homing_player, spawn_trails, despawn_trails).run_if(in_state(GameState::Game)),
        )
        .add_systems(OnExit(GameState::Game), despawn_rainbow);
    }
}

/// Spawns a [Homing].
/// # Arguments
/// * `x` - if [None], a random `x` within resolution is chosen.
/// * `y` - if [None], it is set 100px above upper bound.
/// * `commands` - [Commands].
/// * `texture` - Handle for rainbow [TextureAtlas].
pub fn create_rainbow(
    x: Option<f32>,
    y: Option<f32>,
    commands: &mut Commands,
    texture: &Handle<TextureAtlas>,
) {
    let mut sprite = TextureAtlasSprite::new(0);
    sprite.custom_size = Some(Vec2::new(65., 1.));

    let x = x.unwrap_or((rand::random::<f32>() - 0.5) * (1920. / 3.));
    let y = y.unwrap_or(500.);

    let rainbow = commands
        .spawn(SpriteSheetBundle {
            sprite,
            texture_atlas: texture.clone(),
            transform: Transform {
                translation: Vec3::new(x, y, 500. + rand::random::<f32>()),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(TrailTimer(Timer::from_seconds(16.0, TimerMode::Once)))
        .insert(Rainbow)
        .insert(Damaging)
        .insert(Homing)
        .id();

    commands.entity(rainbow);
}

/// Spawns [TrailTimer] [entities](Entity) on the same coordinates as [Homing].
/// # Arguments
/// * `commands` - [Commands].
/// * `texture` - [Resource] containing handle for rainbow [TextureAtlas].
/// * `time` - [time].
/// * `homing_query` - [Query] for a [Homing].
fn spawn_trails(
    mut commands: Commands,
    texture: Res<RainbowSheet>,
    time: Res<Time>,
    homing_query: Query<&Transform, With<Homing>>,
) {
    if time.elapsed_seconds() % 0.015 < time.delta_seconds() {
        for trans in &homing_query {
            let mut sprite = TextureAtlasSprite::new(0);
            sprite.custom_size = Some(Vec2::new(65., 25.));
            let mut trail_trans = *trans;
            trail_trans.translation.z = 600. + time.elapsed_seconds() % 300.; //avoids buggy overlapping
            let player = commands
                .spawn(SpriteSheetBundle {
                    sprite,
                    texture_atlas: texture.0.clone(),
                    transform: trail_trans,
                    ..Default::default()
                })
                .insert(Damaging)
                .insert(RigidBody::Fixed)
                .insert(Collider::capsule_x(28., 2.))
                .insert(TrailTimer(Timer::from_seconds(2.0, TimerMode::Once)))
                .insert(Rainbow)
                .id();
            commands.entity(player);
        }
    }
}

/// Spawns [TrailTimer] [entities](Entity) on the same coordinates as [Homing].
/// # Arguments
/// * `player_query` - [Query] for a [Player].
/// * `homing_query` - [Query] for a [Homing].
/// * `time` - [time].
fn homing_player(
    mut player_query: Query<(&Hidden, &Transform), With<Player>>,
    mut homing_query: Query<&mut Transform, (With<Homing>, Without<Player>)>,
    time: Res<Time>,
) {
    for (hidden, transform_player) in player_query.iter_mut() {
        if !hidden.hidden {
            let player_x = transform_player.translation.x;
            let player_y = transform_player.translation.y;
            for mut transform_homing in homing_query.iter_mut() {
                let homing_x = transform_homing.translation.x;
                let homing_y = transform_homing.translation.y;
                let distance = std::cmp::min(
                    point_distance(player_x, player_y, homing_x, homing_y) as i32,
                    220,
                ) as f32;

                let x_attraction = (14000. / (distance * distance)) * (homing_x - player_x);
                let y_attraction = (14000. / (distance * distance)) * (homing_y - player_y);

                //rotate homing rainbow towards player
                transform_homing.rotation = Quat::from_rotation_z(Vec2::angle_between(
                    Vec2 { x: 0., y: 1. },
                    Vec2 {
                        x: x_attraction,
                        y: y_attraction,
                    },
                ));

                //Gravity can get funky when objects are too close
                if x_attraction.abs() < 1500. && y_attraction.abs() < 1500. {
                    transform_homing.translation.x -= x_attraction * time.delta_seconds();
                    transform_homing.translation.y -= y_attraction * time.delta_seconds();
                }
            }
        }
    }
}

/// Despawns [TrailTimer] [entities](Entity) if their timer is up.
/// # Arguments
/// * `commands` - [Commands].
/// * `trail_query` - [Query] for a [TrailTimer].
/// * `time` - [time].
fn despawn_trails(
    mut commands: Commands,
    mut trail_query: Query<(Entity, &mut TrailTimer), With<TrailTimer>>,
    time: Res<Time>,
) {
    for (entity, mut trail_timer) in &mut trail_query {
        trail_timer.tick(time.delta());
        if trail_timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// Despawns [Rainbow] [entities](Entity).
/// # Arguments
/// * `commands` - [Commands].
/// * `trail_query` - [Query] for [Rainbow].
fn despawn_rainbow(mut commands: Commands, mut rainbow_query: Query<Entity, With<Rainbow>>) {
    for entity in &mut rainbow_query {
        commands.entity(entity).despawn();
    }
}
