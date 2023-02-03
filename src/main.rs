use bevy::prelude::*;
use bevy::window::*;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;

pub const CLEAR: Color = Color::rgb(0.75, 0.70, 1.);
pub const RESOLUTION: f32 = 1920. / 1080.;

mod audio;
mod basic;
mod blackhole;
mod clouds;
mod collisions;
mod energybars;
mod falling;
mod homing;
mod map_layout;
mod plane;
mod planet;
mod player;
mod speed;
mod text;

use audio::GameAudioPlugin;
use blackhole::BlackHolePlugin;
use clouds::CloudPlugin;
use collisions::CollPlugin;
use falling::FallPlugin;
use homing::RainbowPlugin;
use map_layout::MapPlugin;
use plane::PlanePlugin;
use player::PlayerPlugin;
use speed::SpeedPlugin;
use text::TextPlugin;
#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Speed {
    pub num: f32,
}

#[derive(Component)]
struct Switch {
    pub num: usize,
}
#[derive(Resource)]
pub struct UnicornSheet(pub Handle<TextureAtlas>);
#[derive(Resource)]
pub struct HolesSheet(pub Handle<TextureAtlas>);
#[derive(Resource)]
pub struct PlanesSheet(pub Handle<TextureAtlas>);
#[derive(Resource)]
pub struct StarsSheet(pub Handle<TextureAtlas>);
#[derive(Resource)]
pub struct PlanetSheet(pub Handle<TextureAtlas>);
#[derive(Resource)]
pub struct EnergySheet(pub Handle<TextureAtlas>);
#[derive(Resource)]
pub struct RainbowSheet(pub Handle<TextureAtlas>);
#[derive(Resource)]
pub struct CloudSheet(pub Handle<TextureAtlas>);
#[derive(Resource)]
pub struct FullChocSheet(pub Handle<TextureAtlas>);
#[derive(Resource)]
pub struct PartChocSheet(pub Handle<TextureAtlas>);
#[derive(Resource)]
pub struct EggSheet(pub Handle<TextureAtlas>);
#[derive(Resource)]
pub struct LollySheet(pub Handle<TextureAtlas>);
#[derive(Resource)]
pub struct LoveSheet(pub Handle<TextureAtlas>);
#[derive(Resource)]
pub struct KofolaSheet(pub Handle<TextureAtlas>);

#[derive(Component)]
pub struct Gravitating {
    strength: f32,
}
#[derive(Component)]
pub struct Damaging;

#[derive(Component, Deref, DerefMut)]
struct FallTimer(Timer);

fn main() {
    println!("Hello, world!");
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_linear())
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "uni corn".to_string(),
                        width: 1920. / 3.,
                        height: 700.,
                        resizable: false,
                        cursor_grab_mode: CursorGrabMode::Confined,
                        cursor_visible: false,
                        position: WindowPosition::At(Vec2::new(100., 1.)),
                        /*             cursor_locked: true,
                        cursor_visible: false, */
                        //mode: WindowMode::Fullscreen,
                        present_mode: PresentMode::Fifo,
                        ..Default::default()
                    },
                    ..Default::default()
                }),
        )
        .add_startup_system(spawn_camera)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_all)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_plugin(BlackHolePlugin)
        .add_plugin(PlanePlugin)
        .add_plugin(MapPlugin)
        .add_plugin(RainbowPlugin)
        .add_plugin(CloudPlugin)
        .add_plugin(CollPlugin)
        .add_plugin(FallPlugin)
        .add_plugin(SpeedPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(GameAudioPlugin)
        .add_plugin(TextPlugin)
        .insert_resource(RapierConfiguration {
            gravity: Vec2::splat(0.),
            ..Default::default()
        })
        .add_startup_system(switch_int)
        .run();
}
fn load_all(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    struct SheetInfo {
        name: &'static str,
        x: f32,
        y: f32,
        columns: usize,
        rows: usize,
        padding: Option<Vec2>,
        offset: Option<Vec2>,
    }
    impl SheetInfo {
        pub fn new(
            name: &'static str,
            x: f32,
            y: f32,
            columns: usize,
            rows: usize,
            padding: Option<Vec2>,
            offset: Option<Vec2>,
        ) -> SheetInfo {
            return SheetInfo {
                name,
                x,
                y,
                columns,
                rows,
                padding,
                offset,
            };
        }
    }
    let init_arr = [
        SheetInfo::new("mraky_full2.png", 1150., 850., 4, 2, None, None),
        SheetInfo::new("duha.png", 21., 1., 1, 1, None, None),
        SheetInfo::new("planet_sheet.png", 100., 100., 15, 1, None, None),
        SheetInfo::new("star_sheet.png", 21., 21., 1, 1, None, None),
        SheetInfo::new("plane_sheet1.png", 322., 108., 2, 1, None, None),
        SheetInfo::new("energy_sheet.png", 243., 117., 3, 1, None, None),
        SheetInfo::new("blackhole_sheet.png", 223., 223., 4, 1, None, None),
        SheetInfo::new("full_choc.png", 96., 44., 2, 1, None, None),
        SheetInfo::new("part_choc.png", 92., 44., 2, 1, None, None),
        SheetInfo::new("lollysheet.png", 49., 98., 4, 1, None, None),
        SheetInfo::new("lovesheet.png", 100., 100., 2, 1, None, None),
        SheetInfo::new("eggsheet.png", 71., 100., 2, 1, None, None),
        SheetInfo::new("kofolasheet.png", 24., 88., 1, 1, None, None),
        SheetInfo::new(
            "unicorn_sheet.png",
            200.,
            250.,
            8,
            1,
            Some(Vec2::splat(10.0)),
            Some(Vec2::splat(10.0)),
        ),
    ];
    for sheet in init_arr {
        let image = assets.load(sheet.name);
        let atlas = TextureAtlas::from_grid(
            image,
            Vec2::new(sheet.x, sheet.y),
            sheet.columns,
            sheet.rows,
            sheet.padding,
            sheet.offset,
        );

        let atlas_handle = texture_atlases.add(atlas);
        match sheet.name {
            "mraky_full2.png" => commands.insert_resource(CloudSheet(atlas_handle)),
            "duha.png" => commands.insert_resource(RainbowSheet(atlas_handle)),
            "star_sheet.png" => commands.insert_resource(StarsSheet(atlas_handle)),
            "plane_sheet1.png" => commands.insert_resource(PlanesSheet(atlas_handle)),
            "energy_sheet.png" => commands.insert_resource(EnergySheet(atlas_handle)),
            "blackhole_sheet.png" => commands.insert_resource(HolesSheet(atlas_handle)),
            "unicorn_sheet.png" => commands.insert_resource(UnicornSheet(atlas_handle)),
            "planet_sheet.png" => commands.insert_resource(PlanetSheet(atlas_handle)),
            "full_choc.png" => commands.insert_resource(FullChocSheet(atlas_handle)),
            "part_choc.png" => commands.insert_resource(PartChocSheet(atlas_handle)),
            "lollysheet.png" => commands.insert_resource(LollySheet(atlas_handle)),
            "lovesheet.png" => commands.insert_resource(LoveSheet(atlas_handle)),
            "eggsheet.png" => commands.insert_resource(EggSheet(atlas_handle)),
            "kofolasheet.png" => commands.insert_resource(KofolaSheet(atlas_handle)),
            _ => {
                panic!("=============FILE NAME MISSING IN MAIN.RS MATCH EXPRESSION!=============");
            }
        };
    }
}

fn switch_int(mut commands: Commands) {
    let switch = commands.spawn(Switch { num: 0 }).id();
    commands.entity(switch);
}

fn spawn_camera(mut comms: Commands) {
    use bevy::render::camera::ScalingMode;

    let mut camera = Camera2dBundle::default();

    camera.projection = OrthographicProjection {
        left: -1.0 * RESOLUTION,
        right: 1.0 * RESOLUTION,
        bottom: -1.0,
        top: 1.0,
        scaling_mode: ScalingMode::None,
        ..Default::default()
    };

    comms.spawn(Camera2dBundle::default());
}
