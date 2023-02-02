use crate::{FallTimer, Gravitating};
use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody};
pub const PLANET_SIZE: f32 = 100.;

#[derive(Component)]
pub struct Planet;

pub fn create_planet(
    x: Option<f32>,
    y: Option<f32>,
    commands: &mut Commands,
    texture: &Handle<TextureAtlas>,
) {
    let random_num: usize = rand::random();
    let mut sprite = TextureAtlasSprite::new(random_num % 15);
    sprite.custom_size = Some(Vec2::splat(PLANET_SIZE));
    let x = if let Some(x) = x {
        x
    } else {
        (rand::random::<f32>() - 0.5) * (1920. / 3.)
    };
    let y = if let Some(y) = y { y } else { 500. };
    let planet = commands
        .spawn(SpriteSheetBundle {
            sprite: sprite.clone(),
            texture_atlas: texture.clone(),
            transform: Transform {
                translation: Vec3::new(x, y, 900.0),
                rotation: Quat::from_rotation_z((random_num % 360) as f32 / 180.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Planet)
        .insert(FallTimer(Timer::from_seconds(6., TimerMode::Once)))
        .insert(RigidBody::Fixed)
        .insert(Gravitating { strength: 1. })
        .insert(Collider::ball(46.))
        .id();
    commands.entity(planet);
}
