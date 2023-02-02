use crate::{Gravitating, StarsSheet, Switch, UnicornSheet};
use bevy::prelude::*;
use bevy_mouse_tracking_plugin::{mouse_motion::MouseMotionPlugin, MouseMotion};
use bevy_rapier2d::prelude::*;
pub struct PlayerPlugin;
#[derive(Component)]
pub struct Player {
    pub hp: u32,
}
#[derive(Component)]
pub struct Hidden {
    pub hidden: bool,
    pub hit: bool,
    pub hit_energy: f32,
    pub energy: f32,
}
#[derive(Component, Deref, DerefMut)]
struct StarTimer(Timer);

#[derive(Component)]
pub struct Star;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    hidden: Hidden,
    collider: Collider,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_plugin(MouseMotionPlugin)
            .add_system(movement)
            .add_system(hide)
            .add_system(gravity_interaction)
            .add_system(spawn_stars)
            .add_system(despawn_stars)
            .add_system(star_movement)
            .add_system(player_was_hit);
    }
}

fn gravity_interaction(
    mut player_query: Query<(&Hidden, &mut Transform), With<Player>>,
    mut hole_query: Query<(&Gravitating, &mut Transform), (With<Gravitating>, Without<Player>)>,
    time: Res<Time>,
) {
    let (hidden, mut transform_player) = player_query.single_mut();
    if !hidden.hidden {
        let player_x = transform_player.translation.x;
        let player_y = transform_player.translation.y;
        for (gravitating, transform_hole) in hole_query.iter_mut() {
            let hole_x = transform_hole.translation.x;
            let hole_y = transform_hole.translation.y;
            let distance = point_distance(player_x, player_y, hole_x, hole_y);
            if distance <= (400.) {
                let x_dist_opposite = (10_000. / (distance * distance)) * (player_x - hole_x);
                let y_dist_opposite = (10_000. / (distance * distance)) * (player_y - hole_y);
                if x_dist_opposite.abs() < 1500. && y_dist_opposite.abs() < 1500. {
                    transform_player.translation.x -=
                        x_dist_opposite * time.delta_seconds() * gravitating.strength;
                    transform_player.translation.y -=
                        y_dist_opposite * time.delta_seconds() * gravitating.strength;
                } else {
                    transform_player.translation = transform_hole.translation;
                }
            }
        }
    }
}

pub fn point_distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    return ((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)).sqrt();
}
fn movement(
    mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    mouse: Res<MouseMotion>,
) {
    let (mut transform, mut velocity) = player_query.single_mut();
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
            transform.translation.x = hori_bound * -1.;
        } else {
            transform.translation.x = hori_bound;
        }
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

fn player_was_hit(
    mut commands: Commands,
    time: Res<Time>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    mut player_query: Query<(&Player, Entity, &mut Hidden, &Transform), With<Player>>,
    mut player_sprite: Query<&mut TextureAtlasSprite, With<Player>>,
    mut switch: Query<&mut Switch, With<Switch>>,
) {
    let (player, e, mut hidden, _transform) = player_query.single_mut();
    let mut sprite = player_sprite.single_mut();
    if hidden.hit {
        if hidden.hit_energy == 150. {
            hidden.hidden = true;
            sprite.index = ((3 - (player.hp % 4)) * 2 + 1) as usize;
            commands.entity(e).remove::<Collider>();
        }
        if hidden.hit_energy % 2. <= 30. * time.delta_seconds() && hidden.hit_energy > 132. {
            let mut transform = camera.single_mut();
            let mut switch = switch.single_mut();
            transform.translation.x += 6. * if switch.num % 2 == 0 { -1. } else { 1. };
            switch.num += 1;
        }
        hidden.hit_energy -= 30. * time.delta_seconds();
        if hidden.hit_energy <= 0. {
            hidden.hit = false;
            hidden.hidden = false;
            sprite.index = ((3 - (player.hp % 4)) * 2) as usize;
            commands.entity(e).insert(Collider::compound(vec![
                (Vec2::new(0., -14.), 0.15, Collider::capsule_x(18., 25.)),
                (Vec2::new(0., -1.), 0., Collider::capsule_y(15., 27.)),
            ]));
        };
    }
}

fn hide(
    buttons: Res<Input<MouseButton>>,
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<(&Player, Entity, &mut Hidden, &Transform), With<Player>>,
    mut query: Query<&mut TextureAtlasSprite, With<Player>>,
) {
    let (player, e, mut hidden, _transform) = player_query.single_mut();
    let mut sprite = query.single_mut();
    let has_energy = hidden.energy > 0.;
    if !hidden.hit {
        if buttons.just_pressed(MouseButton::Left) && has_energy {
            hidden.hidden = true;
            sprite.index = ((3 - (player.hp % 4)) * 2 + 1) as usize;
            commands.entity(e).remove::<Collider>();
        }

        if buttons.pressed(MouseButton::Left) && has_energy {
            hidden.energy -= 30. * time.delta_seconds();
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
            sprite: sprite,
            texture_atlas: uni.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, -200.0, 900.0),
                scale: Vec3::splat(1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Player"))
        .insert(Player { hp: 31 })
        .insert(Hidden {
            hidden: false,
            hit: false,
            hit_energy: 0.,
            energy: 100.,
        })
        .insert(RigidBody::Dynamic)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Velocity {
            linvel: Vec2::splat(0.0),
            angvel: 0.0,
        })
        .insert(Restitution::coefficient(0.0))
        .insert(Friction::coefficient(0.0))
        .insert(physics.0)
        .insert(physics.1)
        .insert(physics.2)
        .insert(Ccd::enabled())
        .id();
    commands.entity(player);
}

fn spawn_stars(
    mut commands: Commands,
    star: Res<StarsSheet>,
    time: Res<Time>,
    player_query: Query<(&Transform, &Hidden), With<Player>>,
) {
    let spawn_chance: f32 = rand::random();
    if time.elapsed_seconds() % 0.2 < time.delta_seconds() && spawn_chance < 0.9 {
        let (player_trans, hidden) = player_query.single();
        let scale_num: f32 = rand::random::<f32>() / 2.;
        let x_num: f32 = rand::random();
        let y_num: f32 = rand::random();
        let color_num: f32 = rand::random();
        let mut sprite = TextureAtlasSprite::new(0);
        sprite.color = bevy::render::color::Color::hsl(color_num * 360. /* 60. */, 0.97, 0.79);
        let star_y = player_trans.translation.y + if hidden.hidden { 0. } else { -40. };
        let player = commands
            .spawn(SpriteSheetBundle {
                sprite: sprite,
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

fn despawn_stars(
    mut commands: Commands,
    mut star_query: Query<(Entity, &mut StarTimer), With<Star>>,
    time: Res<Time>,
) {
    for (entity, mut star_timer) in &mut star_query {
        star_timer.tick(time.delta());
        if star_timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn star_movement(mut star_query: Query<(&Star, &mut Transform), With<Star>>, time: Res<Time>) {
    for (_star, mut transform_hole) in star_query.iter_mut() {
        transform_hole.translation.y -= 40. * time.delta_seconds();
    }
}
