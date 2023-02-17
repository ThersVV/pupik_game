use crate::{falling::FallTimer, AnimationTimer, Gravitating};
use bevy::prelude::*;

/// Size of [Hole] enemies
pub const BLACKHOLE_SIZE: f32 = 70.;

///Labels non-damaging black holes, entities, which suck the player in with a big force (2.5times that of a planet).
#[derive(Component)]
pub struct Hole;

///Spawns a [Hole] object.
/// # Arguments
/// * `x` - if [None], a random `x` within resolution is chosen.
/// * `y` - if [None], it is set 100px above upper bound.
/// * `commands` - [Commands].
/// * `texture` - Handle for black hole [TextureAtlas]
pub fn create_hole(
    x: Option<f32>,
    y: Option<f32>,
    commands: &mut Commands,
    texture: &Handle<TextureAtlas>,
) {
    let mut sprite = TextureAtlasSprite::new(0);
    sprite.custom_size = Some(Vec2::splat(BLACKHOLE_SIZE));

    let x = x.unwrap_or(rand::random::<f32>() - 0.5) * (1920. / 3.);
    let y = y.unwrap_or(500.);

    let hole = commands
        .spawn(SpriteSheetBundle {
            sprite,
            texture_atlas: texture.clone(),
            transform: Transform {
                translation: Vec3::new(x, y, 900.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Hole)
        .insert(FallTimer(Timer::from_seconds(6., TimerMode::Once)))
        .insert(Gravitating { strength: 2.5 })
        .insert(AnimationTimer(Timer::from_seconds(
            0.15,
            TimerMode::Repeating,
        )))
        .id();
    commands.entity(hole);
}
