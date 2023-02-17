use crate::falling::FallTimer;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

///Labels [entities](Entity) which when collided with give the player instantly some amount of energy.
#[derive(Component)]
pub struct EnergyBar;

/// Spawns an [EnergyBar].
/// # Arguments
/// * `x` - if [None], a random `x` within resolution is chosen.
/// * `y` - if [None], it is set 100px above upper bound.
/// * `commands` - [Commands].
/// * `texture` - Handle for energybar [TextureAtlas]
pub fn create_bar(
    x: Option<f32>,
    y: Option<f32>,
    commands: &mut Commands,
    texture: &Handle<TextureAtlas>,
) {
    let random_num: usize = rand::random();
    let sprite = TextureAtlasSprite::new(random_num % 3);
    let x = x.unwrap_or(rand::random::<f32>() - 0.5) * (1920. / 3.);
    let y = y.unwrap_or(500.);
    let bar = commands
        .spawn(SpriteSheetBundle {
            sprite,
            texture_atlas: texture.clone(),
            transform: Transform {
                translation: Vec3::new(x, y, 900.0),
                scale: Vec3::splat(0.6),
                rotation: Quat::from_rotation_z((random_num % 360) as f32 / 180.),
            },
            ..Default::default()
        })
        .insert(EnergyBar)
        .insert(Collider::cuboid(121., 59.))
        .insert(FallTimer(Timer::from_seconds(8.85, TimerMode::Once)))
        .insert(Sensor)
        .id();
    commands.entity(bar);
}
