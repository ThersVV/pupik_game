use crate::{falling::FallTimer, speed::Speed, CloudSheet};
use bevy::prelude::*;

///[Plugin] taking care of background [Cloud] functionalities
pub struct CloudPlugin;

///Labels clouds, [entities](Entity) with no collision and low z coordinate
#[derive(Component)]
pub struct Cloud;

impl Plugin for CloudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_clouds);
    }
}

///Labels clouds, [entities](Entity) with no collision and low z coordinate
/// # Arguments
/// * `commands` - [Commands].
/// * `cloud_sheet` - [Resource] containing handle for cloud [TextureAtlas].
/// * `time` - [Time]. Used to spawn clouds regularly.
/// * `speed` - [Speed]. Used to spawn clouds more often once speed gets high.
fn spawn_clouds(
    mut commands: Commands,
    cloud_sheet: Res<CloudSheet>,
    time: Res<Time>,
    speed: Res<Speed>,
) {
    if time.elapsed_seconds() % (0.5 / speed.speed) < time.delta_seconds() {
        let scale_num: f32 = rand::random::<f32>() / 2.;
        let x_num = rand::random::<f32>() - 0.5;
        let sprite_num: usize = rand::random();
        //let rotation_num = rand::random::<f32>(); <- maybe with better cloud textures
        let sprite = TextureAtlasSprite::new(sprite_num % 8);
        let cloud = commands
            .spawn(SpriteSheetBundle {
                sprite,
                texture_atlas: cloud_sheet.0.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        x_num * 1920. / 3.,
                        600.,
                        100.0 + time.elapsed_seconds() % 200.,
                    ),
                    scale: Vec3::splat(0.3 + scale_num * scale_num), //Bigger chance for smaller clouds looks and runs better
                    //rotation: Quat::from_rotation_z((rotation_num * 360.) as f32 / 180.), <- maybe with better cloud textures
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
