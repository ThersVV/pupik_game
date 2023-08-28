use crate::{
    basic::create_basic,
    blackhole::create_hole,
    energybars::create_bar,
    homing::create_rainbow,
    plane::{create_plane_sensor, PlaneDir},
    planet::create_planet,
    speed::Speed,
    EggSheet, EnergySheet, FullChocSheet, GameState, HolesSheet, KofolaSheet, LollySheet,
    LoveSheet, PartChocSheet, PlanetSheet, RainbowSheet,
};

use bevy::prelude::*;

use std::collections::BTreeSet;

///[Plugin] which takes care of random enemy spawning, later with pre-designed structures and a structure
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), spawn_map)
            .add_systems(OnExit(GameState::Game), despawn_map)
            .add_systems(Update, spawning.run_if(in_state(GameState::Game)));
    }
}

///Used in [SpawnEvent] to differentiate between enemy types.
/// Can have following values:
/// * `HoleE` - [spawning] will call the [create_hole] function
/// * `BarE` - [spawning] will call the [create_bar] function
/// * `RainbowE` - [spawning] will call the [create_rainbow] function
/// * `PlaneE` - [spawning] will call the [create_plane_sensor] function
/// * `BasicE` - [spawning] will call the [create_basic] function
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Enemy {
    HoleE,
    BarE,
    RainbowE,
    PlaneE,
    PlanetE,
    BasicE,
}

///The building block of [Structure].
/// # Fields
/// * `x` - x coordinate of enemy
/// * `y` - y coordinate of enemy
/// * `enemy` - Type of enemy. See [Enemy].
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct SpawnEvent {
    x: Option<i32>,
    y: Option<i32>,
    enemy: Enemy,
}

/// A [BTreeSet] containing all planned events. Currently used in debugging, later will be used for
/// not so random structures in the beggining. Random spawning will be more of an end-game thing.
/// * `spawn_chance` - weight in probability calculation
/// * `structure` - [BTreeSet] of [SpawnEvent].
#[derive(Component)]
struct Structure {
    spawn_chance: f64,
    structure: BTreeSet<SpawnEvent>,
}

#[derive(Component)]
struct Map {
    map: Vec<Structure>,
}

/// Spawns [Map].
/// # Arguments
/// * `commands` - [Commands]
fn spawn_map(mut commands: Commands) {
    use crate::map_layout::Enemy::{BarE, BasicE, HoleE, PlaneE, PlanetE, RainbowE};
    let mut map = Vec::from([]);
    let singletons: [(f64, Enemy); 6] = [
        (7., HoleE),
        (12., BarE),
        (0.2, RainbowE),
        (38., PlaneE),
        (19., PlanetE),
        (119., BasicE),
    ];
    //add import here
    set_to_sums(&mut map);
    for singleton in singletons {
        map.push(Structure {
            spawn_chance: singleton.0,
            structure: BTreeSet::from([SpawnEvent {
                x: None,
                y: None,
                enemy: singleton.1,
            }]),
        });
    }
    let structure = commands.spawn(Map { map }).id();
    commands.entity(structure);
}

fn set_to_sums(vec: &mut Vec<Structure>) {
    let mut sum: f64 = 0.;
    for structure in vec {
        let before_addition = structure.spawn_chance;
        structure.spawn_chance += sum;
        sum += before_addition;
    }
}

/// Despawns [Structure].
/// # Arguments
/// * `commands` - [Commands]
/// * `structure` - [Query] for [Structure]
fn despawn_map(mut commands: Commands, map: Query<Entity, With<Map>>) {
    commands.entity(map.single()).despawn();
}

/// Spawns [Enemy]s from [Structure] based on their `time_ms`. Once [Structure] is empty, spawns random [Enemy]s.
/// Optimalization reducing the number of arguments are being thought through as you read this.
/// # Arguments
/// * `time` - [Time].
/// * `commands` - [Commands].
/// * `planets` - [Resource] containing handle for planet [TextureAtlas].
/// * `hole` - [Resource] containing handle for black hole [TextureAtlas].
/// * `energy_bar` - [Resource] containing handle for energy_bar [TextureAtlas].
/// * `rainbow` - [Resource] containing handle for rainbow [TextureAtlas].
/// * `full_choc` - [Resource] containing handle for full chocolate [TextureAtlas].
/// * `part_choc` - [Resource] containing handle for partial chocolate [TextureAtlas].
/// * `egg` - [Resource] containing handle for chocolate egg [TextureAtlas].
/// * `lolly` - [Resource] containing handle for lollipop [TextureAtlas].
/// * `love` - [Resource] containing handle for round gingerbread [TextureAtlas].
/// * `drink` - [Resource] containing handle for drink [TextureAtlas].
/// * `query` -[Query] for [Structure].
/// * `speed` - [Speed].
fn spawning(
    time: Res<Time>,
    mut commands: Commands,
    planets: Res<PlanetSheet>,
    hole: Res<HolesSheet>,
    energy_bar: Res<EnergySheet>,
    rainbow: Res<RainbowSheet>,
    full_choc: Res<FullChocSheet>,
    part_choc: Res<PartChocSheet>,
    egg: Res<EggSheet>,
    lolly: Res<LollySheet>,
    love: Res<LoveSheet>,
    drink: Res<KofolaSheet>,
    mut query: Query<&mut Map, With<Map>>,
    speed: Res<Speed>,
) {
    let speed = speed.speed;
    if time.elapsed_seconds() % (0.7 / speed) >= time.delta_seconds() {
        return;
    }
    let map = &query.single_mut().map;
    let random_num = rand::random::<f64>() * (map.last().unwrap().spawn_chance);
    for i in 0..map.len() {
        if map[i].spawn_chance < random_num {
            continue;
        }
        for spawn_event in &map[i].structure {
            let (x, y, enemy) = (
                spawn_event.x.map(|x| x as f32),
                spawn_event.y.map(|y| y as f32),
                &spawn_event.enemy,
            );
            match enemy {
                &Enemy::HoleE => create_hole(x, y, &mut commands, &hole.0),

                &Enemy::BarE => create_bar(x, y, &mut commands, &energy_bar.0),

                &Enemy::RainbowE => create_rainbow(x, y, &mut commands, &rainbow.0),
                &Enemy::PlaneE => create_plane_sensor(y, PlaneDir::Right, &mut commands),
                &Enemy::PlanetE => create_planet(x, y, &mut commands, &planets.0),
                &Enemy::BasicE => create_basic(
                    x,
                    y,
                    &mut commands,
                    &full_choc.0,
                    &part_choc.0,
                    &egg.0,
                    &lolly.0,
                    &love.0,
                    &drink.0,
                ),
            }
        }
        break;
    }
}
