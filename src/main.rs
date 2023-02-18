#![windows_subsystem = "windows"]

use bevy::prelude::*;
use bevy::window::*;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;

pub const CLEAR: Color = Color::rgb(0.75, 0.70, 1.);
pub const RESOLUTION: f32 = 1920. / 1080.;

//MODULE DECLARATION
///Audio related functionalities.
/// # Contains
/// * [GameAudioPlugin]
/// * [play_background_music]
mod audio;
///Functionalities corelating with [BasicObject].
/// # Contains
/// * [BasicObject]
/// * [BasicBundle]
/// * [create_basic]
/// * [create_full_choc]
/// * [create_part_choc]
/// * [create_egg]
/// * [create_lolly]
/// * [create_love]
/// * [create_drink]
mod basic;
///Functionalities corelating with [Hole]
/// # Contains
/// * [Hole]
/// * [create_hole]
mod blackhole;
///Functionalities corelating with [Cloud]
/// # Contains
/// * [CloudPlugin]
/// * [Cloud]
/// * [spawn_clouds]
mod clouds;
///Collision functionalities
/// # Contains
/// * [CollPlugin]
/// * [Damaging]
/// * [manage_special_collisions]
/// * [deal_damage]
mod collisions;
///Cursor functionalities
/// # Contains
/// * [CursorPlugin]
/// * [hide_cursor]
/// * [unhide_cursor]
mod cursor;
///Functionalities corelating with [EndScreen]
/// # Contains
/// * [EndScreenPlugin]
/// * [EndScreen]
/// * [ContinueButton]
/// * [spawn_endscreen]
/// * [continue_interaction]
/// * [despawn_endscreen]
mod endscreen;
///Functionalities corelating with [EnergyBar]
/// # Contains
/// * [EnergyBar]
/// * [create_bar]
mod energybars;
///Functionalities corelating with [FallTimer]
/// # Contains
/// * [FallPlugin]
/// * [FallTimer]
/// * [ingame_despawn]
/// * [endscreen_despawn]
/// * [movement]
mod falling;
///Functionalities corelating with [Rainbow]
/// # Contains
/// * [RainbowPlugin]
/// * [Rainbow]
/// * [Homing]
/// * [TrailTimer]
/// * [create_rainbow]
/// * [spawn_trails]
/// * [homing_player]
/// * [despawn_trails]
/// * [despawn_rainbow]
mod homing;
///Functionalities corelating with [MainMenu]
/// # Contains
/// * [MenuPlugin]
/// * [MainMenu]
/// * [TutorialButton]
/// * [PlayButton]
/// * [Flickering]
/// * [LoadTimer]
/// * [spawn_credits]
/// * [spawn_start_text]
/// * [despawn_menu]
/// * [click2play]
/// * [tutorial_button_interaction]
/// * [change_flick]
/// * [load_game]
mod mainmenu;
///Random enemy spawning, later with pre-designed structures and a map
/// # Contains
/// * [MapPlugin]
/// * [Enemy]
/// * [SpawnEvent]
/// * [Map]
/// * [spawn_map]
/// * [despawn_map]
/// * [spawning]
mod map_layout;
///Functionalities corelating with [Plane]
/// # Contains
/// * [PlanePlugin]
/// * [Plane]
/// * [PlaneSensor]
/// * [PlaneDir]
/// * [create_plane_sensor]
/// * [create_plane]
/// * [plane_movement]
/// * [despawn_planes]
/// * [plane_endscreen_despawn]
mod plane;
///Functionalities corelating with [Planet]
/// # Contains
/// * [Planet]
/// * [create_planet]
mod planet;
///Functionalities corelating with [Player]
/// # Contains
/// * [PlayerPlugin]
/// * [Player]
/// * [ScreenShaker]
/// * [Hidden]
/// * [StarTimer]
/// * [Star]
/// * [gravity_interaction]
/// * [point_distance]
/// * [movement]
/// * [movement]
/// * [player_was_hit]
/// * [hide]
/// * [cam_shake]
/// * [spawn_player]
/// * [despawn_player]
/// * [spawn_stars]
/// * [despawn_stars]
/// * [star_movement]
mod player;
///Functionalities corelating with [Speed]
/// # Contains
/// * [SpeedPlugin]
/// * [Speed]
/// * [scaling]
/// * [spawn_tachometer]
/// * [reset_speed]
mod speed;
///Functionalities corelating with ingame [Text]
/// # Contains
/// * [TextPlugin]
/// * [ScoreText]
/// * [Score]
/// * [EnergyText]
/// * [score_counter]
/// * [energy_counter]
/// * [score_update]
/// * [energy_update]
/// * [despawn_text]
/// * [reset_score]
mod text;
///Functionalities corelating with [Tutorial]
/// # Contains
/// * [TutorialPlugin]
/// * [Tutorial]
/// * [BackButton]
/// * [spawn_background]
/// * [despawn_tutorial]
/// * [back_button]
/// * [explain_game]
/// * [backbutton_system]
mod tutorial_screen;

use audio::GameAudioPlugin;
use clouds::CloudPlugin;
use collisions::CollPlugin;
use cursor::CursorPlugin;
use endscreen::EndScreenPlugin;
use falling::FallPlugin;
use homing::RainbowPlugin;
use mainmenu::MenuPlugin;
use map_layout::MapPlugin;
use plane::PlanePlugin;
use player::PlayerPlugin;
use speed::SpeedPlugin;
use text::TextPlugin;
use tutorial_screen::TutorialPlugin;

///Enum containing possible [State]s
/// # Fields
/// * `MainMenu` - When state is set, the main menu loads
/// * `Tutorial` - When state is set, the "How to play" section loads
/// * `Game` - When state is set, the game itself loads
/// * `EndScreen` - When state is set, the end screen loads
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    ///When state is set, the main menu loads
    MainMenu,
    ///When state is set, the "How to play" section loads
    Tutorial,
    ///When state is set, the game itself loads
    Game,
    ///When state is set, the end screen loads
    EndScreen,
}

///A [Timer] used for animations.
#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

///Contains constant values that are used in the code, mainly for better readability, but also debugging and easier changes.
/// # Fields
/// * `startup_speed` - A value at which speed is initialized at the start of the [GameState::Game].
/// * `speed_scaling` - How fast does [Speed] scale with time.
/// * `startup_score` - A value at which score is initialized at the start of the [GameState::Game].
/// * `hit_resistence` - For how long is [Player] hidden after being hit.
/// * `shakes` - How many shakes happen after being hit.
#[derive(Resource)]
struct Settings {
    startup_speed: f32,
    speed_scaling: f32,
    startup_score: f32,
    hit_resistence: f32,
    shakes: usize,
}

///[Handle] for unicorn [TextureAtlas].
#[derive(Resource)]
pub struct UnicornSheet(pub Handle<TextureAtlas>);
///[Handle] for black hole [TextureAtlas].
#[derive(Resource)]
pub struct HolesSheet(pub Handle<TextureAtlas>);
///[Handle] for plane [TextureAtlas].
#[derive(Resource)]
pub struct PlanesSheet(pub Handle<TextureAtlas>);
///[Handle] for star [TextureAtlas].
#[derive(Resource)]
pub struct StarsSheet(pub Handle<TextureAtlas>);
///[Handle] for planet [TextureAtlas].
#[derive(Resource)]
pub struct PlanetSheet(pub Handle<TextureAtlas>);
///[Handle] for energybar [TextureAtlas].
#[derive(Resource)]
pub struct EnergySheet(pub Handle<TextureAtlas>);
///[Handle] for rainbow [TextureAtlas].
#[derive(Resource)]
pub struct RainbowSheet(pub Handle<TextureAtlas>);
///[Handle] for cloud [TextureAtlas].
#[derive(Resource)]
pub struct CloudSheet(pub Handle<TextureAtlas>);
///[Handle] for full chocolate bar [TextureAtlas].
#[derive(Resource)]
pub struct FullChocSheet(pub Handle<TextureAtlas>);
///[Handle] for partial chocolate bar [TextureAtlas].
#[derive(Resource)]
pub struct PartChocSheet(pub Handle<TextureAtlas>);
///[Handle] for chocolate egg [TextureAtlas].
#[derive(Resource)]
pub struct EggSheet(pub Handle<TextureAtlas>);
///[Handle] for lollipop [TextureAtlas].
#[derive(Resource)]
pub struct LollySheet(pub Handle<TextureAtlas>);
///[Handle] for round gingerbread [TextureAtlas].
#[derive(Resource)]
pub struct LoveSheet(pub Handle<TextureAtlas>);
///[Handle] for drink [TextureAtlas].
#[derive(Resource)]
pub struct KofolaSheet(pub Handle<TextureAtlas>);

///Labels [entities](Entity) which suck [Player] closer.
#[derive(Component)]
pub struct Gravitating {
    strength: f32,
}

fn main() {
    App::new()
        .add_state(GameState::MainMenu)
        .insert_resource(ClearColor(CLEAR))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_linear())
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "pupik".to_string(),
                        width: 1920. / 3.,
                        height: 700.,
                        resizable: false,
                        position: WindowPosition::At(Vec2::new(100., 1.)),
                        present_mode: PresentMode::Fifo,
                        ..Default::default()
                    },
                    ..Default::default()
                }),
        )
        .add_startup_system(spawn_camera)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_all)
        .add_system(animate_objects)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(AudioPlugin)
        .add_plugin(CloudPlugin)
        .add_plugin(CollPlugin)
        .add_plugin(CursorPlugin)
        .add_plugin(EndScreenPlugin)
        .add_plugin(FallPlugin)
        .add_plugin(GameAudioPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(PlanePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(RainbowPlugin)
        .add_plugin(SpeedPlugin)
        .add_plugin(TextPlugin)
        .add_plugin(TutorialPlugin)
        .insert_resource(RapierConfiguration {
            gravity: Vec2::splat(0.),
            ..Default::default()
        })
        .insert_resource(Settings {
            startup_score: 120.,
            startup_speed: 1.6,
            speed_scaling: 0.01,
            hit_resistence: 100.,
            shakes: 4,
        })
        .run();
}

///Loads all spritesheets from the assets folder into the [AssetServer]
/// # Arguments
/// * `commands` -[Commands].
/// * `assets` - [AssetServer].
/// * `texture_atlases` - [Assets] of type [TextureAtlas].
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
            SheetInfo {
                name,
                x,
                y,
                columns,
                rows,
                padding,
                offset,
            }
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

/// Spawns the camera.
/// # Arguments
/// * `commands` - [Commands]
fn spawn_camera(mut commands: Commands) {
    use bevy::render::camera::ScalingMode;

    let mut camera = Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(0., 0., 5000.),
            ..default()
        },
        ..default()
    };

    camera.projection = OrthographicProjection {
        left: -1.0 * RESOLUTION,
        right: 1.0 * RESOLUTION,
        bottom: -1.0,
        top: 1.0,
        scaling_mode: ScalingMode::None,
        ..Default::default()
    };

    commands.spawn(Camera2dBundle::default());
}

///Animates [entities](Entity) containing [AnimationTimer].
/// # Arguments
/// * `time` - [Time].
/// * `texture_atlases` - [Assets] of type [TextureAtlas].
/// * `query` - [Query] for [AnimationTimer]
fn animate_objects(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<
        (
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        With<AnimationTimer>,
    >,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}
