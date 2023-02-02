use crate::{
    player::{point_distance, Hidden, Player},
    Damaging, RainbowSheet,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
pub struct RainbowPlugin;

#[derive(Component)]
pub struct Rainbow;

#[derive(Component)]
pub struct Trail;

#[derive(Component)]
pub struct Homing;

#[derive(Component, Deref, DerefMut)]
struct TrailTimer(Timer);

impl Plugin for RainbowPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(homing_player)
            .add_system(spawn_trails)
            .add_system(despawn_trails);
    }
}

pub fn create_rainbow(
    x: Option<f32>,
    y: Option<f32>,
    commands: &mut Commands,
    texture: &Handle<TextureAtlas>,
) {
    let mut sprite = TextureAtlasSprite::new(0);
    sprite.custom_size = Some(Vec2::new(65., 1.));
    let x = if let Some(x) = x {
        x
    } else {
        (rand::random::<f32>() - 0.5) * (1920. / 3.)
    };
    let y = if let Some(y) = y { y } else { 450. };

    let rainbow = commands
        .spawn(SpriteSheetBundle {
            sprite: sprite.clone(),
            texture_atlas: texture.clone(),
            transform: Transform {
                translation: Vec3::new(x, y, 500.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(TrailTimer(Timer::from_seconds(12.0, TimerMode::Once)))
        .insert(Rainbow)
        .insert(Damaging)
        .insert(Homing)
        .id();

    commands.entity(rainbow);
}

fn spawn_trails(
    mut commands: Commands,
    rainbow: Res<RainbowSheet>,
    time: Res<Time>,
    homing_query: Query<&Transform, (With<Homing>, Without<Player>)>,
) {
    if time.elapsed_seconds() % 0.015 < time.delta_seconds() {
        for trans in &homing_query {
            let mut sprite = TextureAtlasSprite::new(0);
            sprite.custom_size = Some(Vec2::new(65., 25.));
            let mut trail_trans = trans.clone();
            trail_trans.translation.z = 600. + time.elapsed_seconds() % 300.;
            let player = commands
                .spawn(SpriteSheetBundle {
                    sprite,
                    texture_atlas: rainbow.0.clone(),
                    transform: trail_trans,
                    ..Default::default()
                })
                .insert(Trail)
                .insert(Damaging)
                .insert(RigidBody::Fixed)
                .insert(Collider::capsule_x(28., 2.))
                .insert(TrailTimer(Timer::from_seconds(2.0, TimerMode::Once)))
                .id();
            commands.entity(player);
        }
    }
}

fn homing_player(
    mut player_query: Query<(&Hidden, &Transform), With<Player>>,
    mut homing_query: Query<&mut Transform, (With<Homing>, Without<Player>)>,
    time: Res<Time>,
) {
    let (hidden, transform_player) = player_query.single_mut();
    if !hidden.hidden {
        let player_x = transform_player.translation.x;
        let player_y = transform_player.translation.y;
        for mut transform_homing in homing_query.iter_mut() {
            let homing_x = transform_homing.translation.x;
            let homing_y = transform_homing.translation.y;
            let distance = std::cmp::min(
                point_distance(player_x, player_y, homing_x, homing_y) as i32,
                150,
            ) as f32;

            if distance <= (2000.) {
                let x_dist_opposite = (14000. / (distance * distance)) * (homing_x - player_x);
                let y_dist_opposite = (14000. / (distance * distance)) * (homing_y - player_y);
                transform_homing.rotation = Quat::from_rotation_z(Vec2::angle_between(
                    Vec2 { x: 0., y: 1. },
                    Vec2 {
                        x: x_dist_opposite as f32,
                        y: y_dist_opposite as f32,
                    },
                ));

                if x_dist_opposite.abs() < 1500. && y_dist_opposite.abs() < 1500. {
                    transform_homing.translation.x -= x_dist_opposite as f32 * time.delta_seconds();
                    transform_homing.translation.y -= y_dist_opposite as f32 * time.delta_seconds();
                }
            }
        }
    }
}

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
