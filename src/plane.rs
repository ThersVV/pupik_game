use crate::{Damaging, FallTimer};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
pub struct PlanePlugin;

#[derive(Component)]
pub struct Plane {
    dir: PlaneDir,
}

#[derive(Component, Copy, Clone)]
pub struct PlaneSensor {
    pub dir: PlaneDir,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(PartialEq, Eq, Component, Clone, Copy)]
pub enum PlaneDir {
    Left,
    Right,
}
impl Plugin for PlanePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(plane_movement).add_system(animate_plane);
    }
}

pub fn create_plane_sensor(y: Option<f32>, dir: PlaneDir, commands: &mut Commands) {
    let y = if let Some(y) = y { y } else { 400. };
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
        .insert(FallTimer(Timer::from_seconds(6., TimerMode::Once)))
        .insert(PlaneSensor { dir })
        .id();
    commands.entity(sensor);
}

pub fn create_plane(dir: PlaneDir, commands: &mut Commands, texture: &Handle<TextureAtlas>) {
    let mut sprite = TextureAtlasSprite::new(0);
    if dir == PlaneDir::Left {
        sprite.flip_x = true;
    } else {
        sprite.flip_x = false;
    }
    let plane = commands
        .spawn(SpriteSheetBundle {
            sprite: sprite.clone(),
            texture_atlas: texture.clone(),
            transform: Transform {
                translation: Vec3::new(
                    (1920. / 6. + 100.) * if dir == PlaneDir::Right { -1. } else { 1. },
                    100.,
                    900.0,
                ),
                scale: Vec3::splat(0.6),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Plane { dir })
        .insert(Damaging)
        .insert(FallTimer(Timer::from_seconds(6., TimerMode::Once)))
        .insert(RigidBody::Fixed)
        .insert(Collider::compound(vec![(
            Vec2::new(0., -17.),
            0.,
            Collider::capsule_x(125., 33.),
        )]))
        .insert(AnimationTimer(Timer::from_seconds(
            0.1,
            TimerMode::Repeating,
        )))
        .id();
    commands.entity(plane);
}

fn plane_movement(mut plane_query: Query<(&mut Transform, &Plane)>, time: Res<Time>) {
    for (mut transform, plane) in plane_query.iter_mut() {
        match plane.dir {
            PlaneDir::Right => transform.translation.x += 200. * time.delta_seconds(),
            PlaneDir::Left => transform.translation.x -= 200. * time.delta_seconds(),
        }
        //slowing down falling
        transform.translation.y += 100. * time.delta_seconds();
    }
}

fn animate_plane(
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
