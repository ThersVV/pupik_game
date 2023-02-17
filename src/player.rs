use crate::{speed::Speed, GameState, Gravitating, Settings, StarsSheet, UnicornSheet};
use bevy::prelude::*;
use bevy_mouse_tracking_plugin::{mouse_motion::MouseMotionPlugin, MouseMotion};
use bevy_rapier2d::prelude::*;

///[Plugin] taking care of functionalities corelating with [Player]
pub struct PlayerPlugin;

///Labels the main [Entity], the player itself. Collision functions only activate when this entity collides.
/// # Fields
/// * `hp` - Remaining hp. Player's sprite changes based on the value of `hp`.
#[derive(Component)]
pub struct Player {
    pub hp: i32,
}

///[Component] that is inserted everytime [Player] collides, activating the [cam_shake] function
/// # Fields
/// * `shakes` - number of camera shakes that should be made. Changable in [Settings].
#[derive(Component)]
pub struct ScreenShaker {
    pub shakes: usize,
}

///Inserted together with [Player], containing his hit info, mainly used
///  for his immunity when player left clicks or gets damaged.
/// # Fields
/// * `hidden` - A bool indicating whether [Player]'s hitbox is off.
/// * `hit` - A bool indicating whether [Player] has been hit recently, turning his hitbox involuntarily off.
/// * `hit_energy` - Counts for how long the player *will* remain `hit`
/// * `energy` - Counts for how long the player *can* remain `hidden`
#[derive(Component)]
pub struct Hidden {
    pub hidden: bool,
    pub hit: bool,
    pub hit_energy: f32,
    pub energy: f32,
}
///Despawn timer for [Star]
#[derive(Component, Deref, DerefMut)]
struct StarTimer(Timer);

///Labels small colorful star [entities](Entity) without hitbox that spawn on and fall a bit [Player].
#[derive(Component)]
pub struct Star;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MouseMotionPlugin)
            .add_system(cam_shake)
            .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(movement)
                    .with_system(hide)
                    .with_system(gravity_interaction)
                    .with_system(player_was_hit),
            )
            .add_system_set(SystemSet::on_exit(GameState::EndScreen).with_system(despawn_player))
            .add_system(spawn_stars)
            .add_system(despawn_stars)
            .add_system(star_movement);
    }
}
///Affects [Player] by [Gravitating] [entities](Entity) like [crate::planet::Planet] or [crate::blackhole::Hole].
/// # Arguments
/// * `player_query` - [Query] for [Player].
/// * `gravitating_query` - [Query] for [Gravitating].
/// * `time` - [Time].
fn gravity_interaction(
    mut player_query: Query<(&Hidden, &mut Transform), With<Player>>,
    mut gravitating_query: Query<
        (&Gravitating, &mut Transform),
        (With<Gravitating>, Without<Player>),
    >,
    time: Res<Time>,
) {
    for (hidden, mut transform_player) in player_query.iter_mut() {
        if !hidden.hidden {
            let player_x = transform_player.translation.x;
            let player_y = transform_player.translation.y;
            for (gravitating, transform_hole) in gravitating_query.iter_mut() {
                let hole_x = transform_hole.translation.x;
                let hole_y = transform_hole.translation.y;
                let distance = point_distance(player_x, player_y, hole_x, hole_y);
                if distance <= 400. {
                    let x_gravity = (10_000. / (distance * distance)) * (player_x - hole_x);
                    let y_gravity = (10_000. / (distance * distance)) * (player_y - hole_y);
                    if x_gravity.abs() < 1500. && y_gravity.abs() < 1500. {
                        transform_player.translation.x -=
                            x_gravity * time.delta_seconds() * gravitating.strength;
                        transform_player.translation.y -=
                            y_gravity * time.delta_seconds() * gravitating.strength;
                    } else {
                        transform_player.translation = transform_hole.translation;
                    }
                }
            }
        }
    }
}
///Returns distance between two points
/// # Arguments
/// * `x1` - x coordinate of the first point
/// * `y1` - y coordinate of the first point
/// * `x2` - x coordinate of the second point
/// * `y2` - y coordinate of the second point
pub fn point_distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)).sqrt()
}

///Moves [Player] based on [MouseMotion].
/// # Arguments
/// * `player_query` - [Query] for [Player].    
/// * `mouse` - [MouseMotion]. Contains easily readable information about mouse movement
fn movement(
    mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    mouse: Res<MouseMotion>,
) {
    for (mut transform, mut velocity) in player_query.iter_mut() {
        let delta = mouse.delta;
        let x = transform.translation.x;
        let y = transform.translation.y;
        let vert_bound = 700. / 2.;
        let hori_bound = 1920. / 6.;
        let is_left = x <= -1. * hori_bound/*  + 192. */;
        let is_right = x >= hori_bound/*  - 192. */;
        let is_down = y <= -1. * vert_bound/*  + 108. */;
        let is_up = y >= vert_bound/*  - 108. */;
        if (!is_right && !is_left)
            || (is_left && velocity.linvel.x >= 0.)
            || (is_right && velocity.linvel.x <= 0.)
        {
            velocity.linvel.x += delta.x * 4.;
        } else {
            if is_right {
                transform.translation.x = hori_bound;
            } else {
                transform.translation.x = hori_bound * -1.;
            }
            velocity.linvel.x = 0.;
        }
        if (!is_down && !is_up)
            || (is_down && velocity.linvel.y >= 0.)
            || (is_up && velocity.linvel.y <= 0.)
        {
            velocity.linvel.y -= delta.y * 4.;
        } else {
            if is_up {
                transform.translation.y = vert_bound;
            } else {
                transform.translation.y = vert_bound * -1.;
            }
            velocity.linvel.y = 0.;
        }
    }
}

/// Handles sprite and hitbox changes when player is hit
/// # Arguments
/// * `commands` - [Commands].
/// * `time` - [Time].
/// * `player_query` - [Query] for [Player].   
/// * `settings` - [Settings].
/// * `buttons` - Mouse input, needed so once [Hidden].hit_energy runs out and left mouse button is pressed, player doesnt unhide
/// for even a frame
fn player_was_hit(
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<(&Player, Entity, &mut TextureAtlasSprite, &mut Hidden), With<Player>>,
    settings: Res<Settings>,
    buttons: Res<Input<MouseButton>>,
) {
    for (player, e, mut sprite, mut hidden) in player_query.iter_mut() {
        if hidden.hit && player.hp >= 0 {
            if hidden.hit_energy == settings.hit_resistence {
                hidden.hidden = true;
                sprite.index = ((3 - (player.hp % 4)) * 2 + 1) as usize;
                commands.entity(e).remove::<Collider>();
            }

            hidden.hit_energy -= 30. * time.delta_seconds();

            if hidden.hit_energy <= 0. {
                hidden.hit = false;
                if !buttons.pressed(MouseButton::Left) {
                    hidden.hidden = false;
                    sprite.index = ((3 - (player.hp % 4)) * 2) as usize;
                    commands.entity(e).insert(Collider::compound(vec![
                        (Vec2::new(0., -14.), 0.15, Collider::capsule_x(18., 25.)),
                        (Vec2::new(0., -1.), 0., Collider::capsule_y(15., 27.)),
                    ]));
                }
            };
        }
    }
}

/// Handles sprite and hitbox changes when player is hit
/// # Arguments
/// * `buttons` - Mouse input.
/// * `commands` - [Commands].
/// * `time` - [Time].
/// * `player_query` - [Query] for [Player].   
fn hide(
    buttons: Res<Input<MouseButton>>,
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<(&Player, Entity, &mut TextureAtlasSprite, &mut Hidden), With<Player>>,
) {
    for (player, e, mut sprite, mut hidden) in player_query.iter_mut() {
        let has_energy = hidden.energy > 0.;
        if !hidden.hit {
            if buttons.just_pressed(MouseButton::Left) && has_energy {
                hidden.hidden = true;
                sprite.index = ((3 - (player.hp % 4)) * 2 + 1) as usize;
                commands.entity(e).remove::<Collider>();
            }

            if buttons.pressed(MouseButton::Left) && has_energy {
                hidden.energy -= 35. * time.delta_seconds();
            } else if hidden.energy < 100. {
                hidden.energy += 4. * time.delta_seconds();
            }

            if buttons.just_released(MouseButton::Left) || !has_energy {
                hidden.hidden = false;
                sprite.index = ((3 - (player.hp % 4)) * 2) as usize;
                commands.entity(e).insert(Collider::compound(vec![
                    (Vec2::new(0., -14.), 0.15, Collider::capsule_x(18., 25.)),
                    (Vec2::new(0., -1.), 0., Collider::capsule_y(15., 27.)),
                ]));
            }
        }
    }
}

///Shakes camera for each [ScreenShaker].
/// # Arguments
/// * `commands` - [Commands].
/// * `time` - [Time].
/// * `camera` - [Query] for [Camera].
/// * `screen_shakers` - [Query] for [ScreenShaker].
fn cam_shake(
    mut commands: Commands,
    time: Res<Time>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    mut screen_shakers: Query<(&mut ScreenShaker, Entity), With<ScreenShaker>>,
) {
    if time.elapsed_seconds() % 0.10 < time.delta_seconds() {
        let mut camera = camera.single_mut();
        for (mut shaker, entity) in &mut screen_shakers {
            camera.translation.x += 6. * if shaker.shakes % 2 == 0 { -1. } else { 1. };
            if shaker.shakes > 0 {
                shaker.shakes -= 1;
            } else {
                commands.entity(entity).despawn();
            }
        }
    }
}

///Spawns [Player].
/// # Arguments
/// * `commands` - [Commands].
/// * `uni` - [Resource] containing unicorn [TextureAtlas].
fn spawn_player(mut commands: Commands, uni: Res<UnicornSheet>) {
    let mut sprite = TextureAtlasSprite::new(0);
    sprite.custom_size = Some(Vec2::new(100., 120.));

    let physics = (
        LockedAxes::ROTATION_LOCKED,
        Damping {
            linear_damping: 6.0,
            angular_damping: 4.0,
        },
        Collider::compound(vec![
            (Vec2::new(-3., -11.), 0.15, Collider::capsule_x(15., 29.)),
            (Vec2::new(0., 2.), 0., Collider::capsule_y(25., 26.)),
        ]),
    );

    let player = commands
        .spawn(SpriteSheetBundle {
            sprite,
            texture_atlas: uni.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, -250.0, 900.0),
                scale: Vec3::splat(1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player { hp: 3 })
        .insert(Hidden {
            hidden: false,
            hit: false,
            hit_energy: 0.,
            energy: 100.,
        })
        .insert(RigidBody::Dynamic)
        .insert(Velocity {
            linvel: Vec2::splat(0.0),
            angvel: 0.0,
        })
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(physics.0)
        .insert(physics.1)
        .insert(physics.2)
        .insert(Ccd::enabled())
        .id();
    commands.entity(player);
}

///Despawns [Player] on exit from [GameState::EndScreen].
/// # Arguments
/// * `commands` - [Commands].
/// * `player_query` - [Query] for [Player].
pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    for entity in player_query.iter() {
        commands.entity(entity).despawn();
    }
}

///Spawns [Star] on [Player] with different colors and slightly different x, y and sizes.
/// # Arguments
/// * `commands` - [Commands].
/// * `star` - [Resource] containing handle for star [TextureAtlas].
/// * `time` - [Time].
/// * `player_query` - [Query] for [Player].
fn spawn_stars(
    mut commands: Commands,
    star: Res<StarsSheet>,
    time: Res<Time>,
    player_query: Query<(&Transform, &Hidden), With<Player>>,
) {
    let spawn_chance: f32 = rand::random();
    if time.elapsed_seconds() % 0.2 < time.delta_seconds() && spawn_chance < 0.9 {
        for (player_trans, hidden) in player_query.iter() {
            let scale_num: f32 = rand::random::<f32>() / 2.;
            let x_num: f32 = rand::random();
            let y_num: f32 = rand::random();
            let color_num: f32 = rand::random();
            let mut sprite = TextureAtlasSprite::new(0);
            sprite.color =
                bevy::render::color::Color::hsl(color_num * 360. /* 60. */, 0.97, 0.79);
            let star_y = player_trans.translation.y + if hidden.hidden { 0. } else { -40. };
            let player = commands
                .spawn(SpriteSheetBundle {
                    sprite,
                    texture_atlas: star.0.clone(),
                    transform: Transform {
                        translation: Vec3::new(
                            player_trans.translation.x - 40. + x_num * 80.,
                            star_y + 15. + 5. * y_num,
                            890.0,
                        ),
                        scale: Vec3::splat(0.5 + scale_num * scale_num),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Star)
                .insert(StarTimer(Timer::from_seconds(1.35, TimerMode::Once)))
                .id();
            commands.entity(player);
        }
    }
}

///Despawns stars.
/// # Arguments
/// * `commands` - [Commands].
/// * `star_query` - [Query] for [StarTimer].
/// * `time` - [Time].
fn despawn_stars(
    mut commands: Commands,
    mut star_timer_query: Query<(Entity, &mut StarTimer), With<StarTimer>>,
    time: Res<Time>,
) {
    for (entity, mut star_timer) in &mut star_timer_query {
        star_timer.tick(time.delta());
        if star_timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

///Moves stars vertically, speeds up with time
/// # Arguments
/// * `star_query` - [Query] for [Star].
/// * `time` - [Time].
/// * `speed` - [Speed].
fn star_movement(
    mut star_query: Query<&mut Transform, With<Star>>,
    time: Res<Time>,
    speed: Res<Speed>,
) {
    for (mut transform_hole) in star_query.iter_mut() {
        transform_hole.translation.y -= 40. * time.delta_seconds() * speed.speed;
    }
}
