use crate::FallTimer;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Bar;

pub fn create_bar(
    x: Option<f32>,
    y: Option<f32>,
    commands: &mut Commands,
    texture: &Handle<TextureAtlas>,
) {
    let random_num: usize = rand::random();
    let sprite = TextureAtlasSprite::new(random_num % 3);
    let x = if let Some(x) = x {
        x
    } else {
        (rand::random::<f32>() - 0.5) * (1920. / 3.)
    };
    let y = if let Some(y) = y { y } else { 400. };
    let bar = commands
        .spawn(SpriteSheetBundle {
            sprite: sprite.clone(),
            texture_atlas: texture.clone(),
            transform: Transform {
                translation: Vec3::new(x, y, 900.0),
                scale: Vec3::splat(0.6),
                rotation: Quat::from_rotation_z((random_num % 360) as f32 / 180.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Bar)
        .insert(Collider::cuboid(121., 59.))
        .insert(FallTimer(Timer::from_seconds(8.85, TimerMode::Once)))
        .insert(Sensor)
        .id();
    commands.entity(bar);
}
