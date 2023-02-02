use crate::{CloudSheet, FallTimer};
use bevy::prelude::*;
pub struct CloudPlugin;

#[derive(Component)]
pub struct Cloud;

#[derive(Component, Deref, DerefMut)]
struct CloudTimer(Timer);

impl Plugin for CloudPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_clouds);
    }
}

fn spawn_clouds(mut commands: Commands, cloud: Res<CloudSheet>, time: Res<Time>) {
    if time.elapsed_seconds_f64() % 0.5 < time.delta_seconds_f64() {
        let scale_num: f32 = rand::random::<f32>() / 2.;
        let x_num = rand::random::<f32>() - 0.5;
        let sprite_num: usize = rand::random();
        //let rotation_num = rand::random::<f32>();
        let sprite = TextureAtlasSprite::new(sprite_num % 8);
        let cloud = commands
            .spawn(SpriteSheetBundle {
                sprite: sprite,
                texture_atlas: cloud.0.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        x_num * 1920. / 3.,
                        600.,
                        100.0 + time.elapsed_seconds() % 200.,
                    ),
                    scale: Vec3::splat(0.3 + scale_num * scale_num),
                    //rotation: Quat::from_rotation_z((rotation_num * 360.) as f32 / 180.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Cloud)
            .insert(FallTimer(Timer::from_seconds(6., TimerMode::Once)))
            .id();
        commands.entity(cloud);
    }
}
