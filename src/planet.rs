use crate::{falling::FallTimer, Gravitating, Object};
use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody};
pub const PLANET_SIZE: f32 = 120.;

///Labels a planet [Entity], a non-damaging solid ball enemy with [Gravitating] property.
#[derive(Component)]
pub struct Planet;

///Spawns a [Planet].
/// # Arguments
/// * `x` - if [None], a random `x` within resolution is chosen.
/// * `y` - if [None], it is set 100px above upper bound.
/// * `commands` - [Commands].
/// * `texture` - [Handle] for planet [TextureAtlas].
pub fn create_planet(
    x: Option<f32>,
    y: Option<f32>,
    commands: &mut Commands,
    texture: &Handle<TextureAtlas>,
) {
    let random_num: usize = rand::random();
    let mut sprite = TextureAtlasSprite::new(random_num % 15);
    sprite.custom_size = Some(Vec2::splat(PLANET_SIZE));
    let x = x.unwrap_or((rand::random::<f32>() - 0.5) * (1920. / 3.));
    let y = y.unwrap_or(600.);
    let planet = commands
        .spawn(SpriteSheetBundle {
            sprite,
            texture_atlas: texture.clone(),
            transform: Transform {
                translation: Vec3::new(x, y, 900. + rand::random::<f32>()),
                rotation: Quat::from_rotation_z((random_num % 360) as f32 / 180.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Planet)
        .insert(FallTimer(Timer::from_seconds(7., TimerMode::Once)))
        .insert(RigidBody::Fixed)
        .insert(Gravitating { strength: 1. })
        .insert(Collider::ball(57.))
        .insert(Object)
        .id();
    commands.entity(planet);
}
