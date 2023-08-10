use crate::{collisions::Damaging, falling::FallTimer, speed::Speed, AnimationTimer, GameState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

///[Plugin] taking care of functionalities corelating with [Plane]
pub struct PlanePlugin;

///Labels planes, damaging [entities](Entity) that spawn by colliding [PlaneSensor] and fly over in given direction.
/// # Fields
/// * `dir` - [PlaneDir].
/// * `timer` - [Timer] used for despawning.
#[derive(Component)]
pub struct Plane {
    dir: PlaneDir,
    timer: Timer,
}

///Spawns a [Plane] when collided with with given direction. Its width is "infinite"
/// # Fields
/// * `dir` - [PlaneDir] direction of the [Plane] it spawns.
#[derive(Component, Copy, Clone)]
pub struct PlaneSensor {
    pub dir: PlaneDir,
}

///Specifies the direction of a [Plane].
/// # Fields
/// * `Left`
/// * `Right`
#[derive(PartialEq, Eq, Component, Clone, Copy)]
pub enum PlaneDir {
    Left,
    Right,
}

impl Plugin for PlanePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (plane_movement, despawn_planes).run_if(in_state(GameState::Game)),
        )
        .add_systems(OnExit(GameState::EndScreen), plane_endscreen_despawn)
        .add_systems(Update, plane_movement);
    }
}

///Spawns a [PlaneSensor].
/// # Arguments
/// * `y` - if [None], it is set 100px above upper bound.
/// * `dir` - [PlaneDir]
/// * `commands` - [Commands]
pub fn create_plane_sensor(y: Option<f32>, dir: PlaneDir, commands: &mut Commands) {
    let y = y.unwrap_or(500.);
    //no optional custom x because its hitbox is "infinitely" long
    let sensor = commands
        .spawn(TransformBundle {
            local: Transform {
                translation: Vec3::new(0., y, 900.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::cuboid(2000., 0.1))
        .insert(Sensor)
        .insert(FallTimer(Timer::from_seconds(10., TimerMode::Once)))
        .insert(PlaneSensor { dir })
        .id();
    commands.entity(sensor);
}

///Spawns a [Plane].
/// # Arguments
/// * `y` - if [None], it is set 100px above upper bound.
/// * `dir` - [PlaneDir].
/// * `commands` - [Commands].
pub fn create_plane(
    dir: PlaneDir,
    y: f32,
    commands: &mut Commands,
    texture: &Handle<TextureAtlas>,
) {
    let mut sprite = TextureAtlasSprite::new(0);
    if dir == PlaneDir::Left {
        sprite.flip_x = true;
    } else {
        sprite.flip_x = false;
    }
    let plane = commands
        .spawn(SpriteSheetBundle {
            sprite,
            texture_atlas: texture.clone(),
            transform: Transform {
                translation: Vec3::new(
                    (1920. / 6. + 100.) * if dir == PlaneDir::Right { -1. } else { 1. },
                    y + 300.,
                    900.0,
                ),
                scale: Vec3::splat(0.6),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Plane {
            dir,
            timer: Timer::from_seconds(3.5, TimerMode::Once),
        })
        .insert(Damaging)
        .insert(RigidBody::Fixed)
        .insert(Collider::compound(vec![(
            Vec2::new(0., -17.),
            0.,
            Collider::capsule_x(125., 33.),
        )]))
        .insert(AnimationTimer(Timer::from_seconds(
            0.2,
            TimerMode::Repeating,
        )))
        .id();
    commands.entity(plane);
}

///Moves [Plane] both vertically down and horizontally based on its `dir`. Vertical velocity is slower than of [FallTimer] objects.
/// # Arguments
/// * `plane_query` - [Query] for [Plane].
/// * `time` - [Time].
/// * `speed` - [Speed]. Only speeds up *vertical* velocity.
fn plane_movement(
    mut plane_query: Query<(&mut Transform, &Plane)>,
    time: Res<Time>,
    speed: Res<Speed>,
) {
    let speed = speed.speed;
    for (mut transform, plane) in plane_query.iter_mut() {
        match plane.dir {
            PlaneDir::Right => transform.translation.x += 200. * time.delta_seconds(),
            PlaneDir::Left => transform.translation.x -= 200. * time.delta_seconds(),
        }
        //needs to fall slower and despawn not based on speed
        transform.translation.y -= 100. * time.delta_seconds() * speed;
    }
}

///Despawns [Plane]s once they leave th screen.
/// # Arguments
/// * `commands` - [Commands].
/// * `query` - [Query] for [Plane].
/// * `time` - [Time].
fn despawn_planes(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Plane), With<Plane>>,
    time: Res<Time>,
) {
    for (entity, mut plane) in &mut query {
        plane.timer.tick(time.delta());
        if plane.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

///Despawns all [Plane] objects on exit from [GameState::EndScreen].
/// # Arguments
/// * `commands` - Commands
/// * `query` - [Query] for [Plane].
fn plane_endscreen_despawn(mut commands: Commands, query: Query<Entity, With<Plane>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
