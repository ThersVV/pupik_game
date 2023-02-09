use crate::{FallTimer, GameState, Gravitating};
use bevy::prelude::*;
pub const BLACKHOLE_SIZE: f32 = 100.;
pub struct BlackHolePlugin;

#[derive(Component)]
pub struct Hole;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

impl Plugin for BlackHolePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Game).with_system(animate_hole));
    }
}

pub fn create_hole(
    x: Option<f32>,
    y: Option<f32>,
    commands: &mut Commands,
    texture: &Handle<TextureAtlas>,
) {
    let mut sprite = TextureAtlasSprite::new(0);
    sprite.custom_size = Some(Vec2::splat(BLACKHOLE_SIZE));

    let x = if let Some(x) = x {
        x
    } else {
        (rand::random::<f32>() - 0.5) * (1920. / 3.)
    };
    let y = if let Some(y) = y { y } else { 500. };
    let hole = commands
        .spawn(SpriteSheetBundle {
            sprite: sprite.clone(),
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

fn animate_hole(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}
