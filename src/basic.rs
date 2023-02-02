use crate::Damaging;
use crate::FallTimer;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
#[derive(Component)]
pub struct BasicObject;

#[derive(Bundle)]
struct BasicBundle {
    object: BasicObject,
    timer: FallTimer,
    body: RigidBody,
    dmg: Damaging,
}

pub fn create(
    x: Option<f32>,
    y: Option<f32>,
    commands: &mut Commands,
    full_choc_t: &Handle<TextureAtlas>,
    part_choc_t: &Handle<TextureAtlas>,
    egg_t: &Handle<TextureAtlas>,
    lolly_t: &Handle<TextureAtlas>,
    love_t: &Handle<TextureAtlas>,
    drink_t: &Handle<TextureAtlas>,
) {
    let random_num: usize = rand::random();
    let x = if let Some(x) = x {
        x
    } else {
        (rand::random::<f32>() - 0.5) * (1920. / 3.)
    };
    let y = if let Some(y) = y { y } else { 500. };

    let bundle = BasicBundle {
        object: BasicObject,
        timer: FallTimer(Timer::from_seconds(6., TimerMode::Once)),
        body: RigidBody::Fixed,
        dmg: Damaging,
    };

    let transform = Transform {
        translation: Vec3::new(x, y, 900.0),
        rotation: Quat::from_rotation_z((random_num % 360) as f32 / 180.),
        ..Default::default()
    };

    let object = match random_num % 6 {
        0 => create_full_choc(transform, bundle, full_choc_t, commands),
        1 => create_part_choc(transform, bundle, part_choc_t, commands),
        2 => create_egg(transform, bundle, egg_t, commands),
        3 => create_lolly(transform, bundle, lolly_t, commands),
        4 => create_love(transform, bundle, love_t, commands),
        5 => create_drink(transform, bundle, drink_t, commands),
        _ =>
        /* never happens */
        {
            create_drink(transform, bundle, drink_t, commands)
        }
    };

    commands.entity(object);
}

fn create_full_choc(
    transform: Transform,
    bundle: BasicBundle,
    texture: &Handle<TextureAtlas>,

    commands: &mut Commands,
) -> Entity {
    let sprite = TextureAtlasSprite::new(rand::random::<usize>() % 2);
    commands
        .spawn(bundle)
        .insert(SpriteSheetBundle {
            sprite: sprite.clone(),
            texture_atlas: texture.clone(),
            transform,
            ..Default::default()
        })
        .insert(Collider::cuboid(48., 22.))
        .id()
}

fn create_part_choc(
    transform: Transform,
    bundle: BasicBundle,
    texture: &Handle<TextureAtlas>,

    commands: &mut Commands,
) -> Entity {
    let sprite = TextureAtlasSprite::new(rand::random::<usize>() % 2);
    commands
        .spawn(bundle)
        .insert(SpriteSheetBundle {
            sprite: sprite.clone(),
            texture_atlas: texture.clone(),
            transform,
            ..Default::default()
        })
        .insert(Collider::compound(vec![(
            Vec2::new(-5., 0.),
            0.,
            Collider::cuboid(40., 22.),
        )]))
        .id()
}

fn create_egg(
    transform: Transform,
    bundle: BasicBundle,
    texture: &Handle<TextureAtlas>,

    commands: &mut Commands,
) -> Entity {
    let sprite = TextureAtlasSprite::new(rand::random::<usize>() % 2);
    commands
        .spawn(bundle)
        .insert(SpriteSheetBundle {
            sprite: sprite.clone(),
            texture_atlas: texture.clone(),
            transform,
            ..Default::default()
        })
        .insert(Collider::capsule_y(20., 27.))
        .id()
}

fn create_lolly(
    transform: Transform,
    bundle: BasicBundle,
    texture: &Handle<TextureAtlas>,

    commands: &mut Commands,
) -> Entity {
    let sprite = TextureAtlasSprite::new(rand::random::<usize>() % 4);
    commands
        .spawn(bundle)
        .insert(SpriteSheetBundle {
            sprite: sprite.clone(),
            texture_atlas: texture.clone(),
            transform,
            ..Default::default()
        })
        .insert(Collider::compound(vec![
            (Vec2::new(0., 25.), 0., Collider::ball(25.)),
            (Vec2::new(0., -10.), 0., Collider::cuboid(3., 37.)),
        ]))
        .id()
}

fn create_love(
    transform: Transform,
    bundle: BasicBundle,
    texture: &Handle<TextureAtlas>,

    commands: &mut Commands,
) -> Entity {
    let sprite = TextureAtlasSprite::new(rand::random::<usize>() % 2);
    commands
        .spawn(bundle)
        .insert(SpriteSheetBundle {
            sprite: sprite.clone(),
            texture_atlas: texture.clone(),
            transform,
            ..Default::default()
        })
        .insert(Collider::ball(49.))
        .id()
}

fn create_drink(
    transform: Transform,
    bundle: BasicBundle,
    texture: &Handle<TextureAtlas>,

    commands: &mut Commands,
) -> Entity {
    let sprite = TextureAtlasSprite::new(rand::random::<usize>() % 1);
    commands
        .spawn(bundle)
        .insert(SpriteSheetBundle {
            sprite: sprite.clone(),
            texture_atlas: texture.clone(),
            transform,
            ..Default::default()
        })
        .insert(Collider::cuboid(10., 40.))
        .id()
}
