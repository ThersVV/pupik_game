use crate::collisions::Damaging;
use crate::falling::FallTimer;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

///Labels falling damaging [entities](Entity) which have no special effects.
#[derive(Component)]
pub struct BasicObject;

///Bundle containing common components of [`BasicObject`].
/// # Fields
/// * `object` - [BasicObject]
/// * `timer` - [FallTimer]
/// * `body` - [RigidBody]
/// * `dmg` - [Damaging]
#[derive(Bundle)]
struct BasicBundle {
    object: BasicObject,
    timer: FallTimer,
    body: RigidBody,
    dmg: Damaging,
}
/// Spawns [BasicObject] object.
/// Calls one of the following function:
/// * [create_full_choc]
/// * [create_part_choc]
/// * [create_egg]
/// * [create_lolly]
/// * [create_love]
/// * [create_drink]
/// # Arguments
/// * `x` - if [None], a random `x` within resolution is chosen.
/// * `y` - if [None], it is set 100px above upper bound.
/// * `commands` - [Commands].
/// * `full_choc_t` ... `drink_t` - [Handle]s for different [TextureAtlas].

pub fn create_basic(
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
    let x = x.unwrap_or((rand::random::<f32>() - 0.5) * (1920. / 3.));
    let y = y.unwrap_or(500.);

    let basic_bundle = BasicBundle {
        object: BasicObject,
        timer: FallTimer(Timer::from_seconds(6., TimerMode::Once)),
        body: RigidBody::Fixed,
        dmg: Damaging,
    };

    let transform = Transform {
        translation: Vec3::new(x, y, 900.),
        rotation: Quat::from_rotation_z((random_num % 360) as f32 / 180.),
        ..Default::default()
    };

    let object = match random_num % 6 {
        0 => create_full_choc(transform, basic_bundle, full_choc_t, commands),
        1 => create_part_choc(transform, basic_bundle, part_choc_t, commands),
        2 => create_egg(transform, basic_bundle, egg_t, commands),
        3 => create_lolly(transform, basic_bundle, lolly_t, commands),
        4 => create_love(transform, basic_bundle, love_t, commands),
        5 => create_drink(transform, basic_bundle, drink_t, commands),
        _ =>
        /* never happens */
        {
            create_drink(transform, basic_bundle, drink_t, commands)
        }
    };

    commands.entity(object);
}

/// Creates full chocolate bar [BasicObject].
/// #Arguments
/// * `transform` - Enemy's [Transform].
/// * `basic_bundle` - [BasicBundle].
/// * `texture` - [Handle] for enemy's [TextureAtlas].
/// * `commands`- [Commands].
fn create_full_choc(
    transform: Transform,
    basic_bundle: BasicBundle,
    texture: &Handle<TextureAtlas>,

    commands: &mut Commands,
) -> Entity {
    let sprite = TextureAtlasSprite::new(rand::random::<usize>() % 2);
    commands
        .spawn(basic_bundle)
        .insert(SpriteSheetBundle {
            sprite,
            texture_atlas: texture.clone(),
            transform,
            ..Default::default()
        })
        .insert(Collider::cuboid(48., 22.))
        .id()
}

/// Creates partial chocolate bar [BasicObject].
/// #Arguments
/// * `transform` - Enemy's [Transform].
/// * `basic_bundle` - [BasicBundle].
/// * `texture` - [Handle] for enemy's [TextureAtlas].
/// * `commands`- [Commands].
fn create_part_choc(
    transform: Transform,
    basic_bundle: BasicBundle,
    texture: &Handle<TextureAtlas>,
    commands: &mut Commands,
) -> Entity {
    let sprite = TextureAtlasSprite::new(rand::random::<usize>() % 2);
    commands
        .spawn(basic_bundle)
        .insert(SpriteSheetBundle {
            sprite,
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

/// Creates chocolate egg [BasicObject].
/// #Arguments
/// * `transform` - Enemy's [Transform].
/// * `basic_bundle` - [BasicBundle].
/// * `texture` - [Handle] for enemy's [TextureAtlas].
/// * `commands`- [Commands].
fn create_egg(
    transform: Transform,
    basic_bundle: BasicBundle,
    texture: &Handle<TextureAtlas>,

    commands: &mut Commands,
) -> Entity {
    let sprite = TextureAtlasSprite::new(rand::random::<usize>() % 2);
    commands
        .spawn(basic_bundle)
        .insert(SpriteSheetBundle {
            sprite,
            texture_atlas: texture.clone(),
            transform,
            ..Default::default()
        })
        .insert(Collider::capsule_y(20., 27.))
        .id()
}

/// Creates lollipop [BasicObject].
/// #Arguments
/// * `transform` - Enemy's [Transform].
/// * `basic_bundle` - [BasicBundle].
/// * `texture` - [Handle] for enemy's [TextureAtlas].
/// * `commands`- [Commands].
fn create_lolly(
    transform: Transform,
    basic_bundle: BasicBundle,
    texture: &Handle<TextureAtlas>,

    commands: &mut Commands,
) -> Entity {
    let sprite = TextureAtlasSprite::new(rand::random::<usize>() % 4);
    commands
        .spawn(basic_bundle)
        .insert(SpriteSheetBundle {
            sprite,
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

/// Creates round gingerbread [BasicObject].
/// #Arguments
/// * `transform` - Enemy's [Transform].
/// * `basic_bundle` - [BasicBundle].
/// * `texture` - [Handle] for enemy's [TextureAtlas].
/// * `commands`- [Commands].
fn create_love(
    transform: Transform,
    basic_bundle: BasicBundle,
    texture: &Handle<TextureAtlas>,

    commands: &mut Commands,
) -> Entity {
    let sprite = TextureAtlasSprite::new(rand::random::<usize>() % 2);
    commands
        .spawn(basic_bundle)
        .insert(SpriteSheetBundle {
            sprite,
            texture_atlas: texture.clone(),
            transform,
            ..Default::default()
        })
        .insert(Collider::ball(49.))
        .id()
}

/// Creates drink [BasicObject].
/// #Arguments
/// * `transform` - Enemy's [Transform].
/// * `basic_bundle` - [BasicBundle].
/// * `texture` - [Handle] for enemy's [TextureAtlas].
/// * `commands`- [Commands].
fn create_drink(
    transform: Transform,
    basic_bundle: BasicBundle,
    texture: &Handle<TextureAtlas>,

    commands: &mut Commands,
) -> Entity {
    let sprite = TextureAtlasSprite::new(rand::random::<usize>() % 1);
    commands
        .spawn(basic_bundle)
        .insert(SpriteSheetBundle {
            sprite,
            texture_atlas: texture.clone(),
            transform,
            ..Default::default()
        })
        .insert(Collider::cuboid(10., 40.))
        .id()
}
