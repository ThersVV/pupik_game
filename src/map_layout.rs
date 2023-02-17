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

///[Plugin] which takes care of random enemy spawning, later with pre-designed structures and a map
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(spawn_map))
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(despawn_map))
            .add_system_set(SystemSet::on_update(GameState::Game).with_system(spawning));
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

///The building block of [Map].
/// # Fields
/// * `time_ms` - time when enemy should spawn
/// * `x` - x coordinate of enemy
/// * `enemy` - Type of enemy. See [Enemy].
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct SpawnEvent {
    time_ms: u64,
    x: i32,
    enemy: Enemy,
}

/// A [BTreeSet] containing all planned events. Currently used in debugging, later will be used for
/// not so random structures in the beggining. Random spawning will be more of an end-game thing.
#[derive(Component)]
struct Map {
    map: BTreeSet<SpawnEvent>,
}

/// Spawns [Map].
/// # Arguments
/// * `commands` - [Commands]
fn spawn_map(mut commands: Commands) {
    let map = commands
        .spawn(Map {
            map: BTreeSet::from([
                 /* TODO: Add planned structures */
            ]),
        })
        .id();
    commands.entity(map);
}

/// Despawns [Map].
/// # Arguments
/// * `commands` - [Commands]
/// * `map` - [Query] for [Map]
fn despawn_map(mut commands: Commands, map: Query<Entity, With<Map>>) {
    commands.entity(map.single()).despawn();
}

/// Spawns [Enemy]s from [Map] based on their `time_ms`. Once [Map] is empty, spawns random [Enemy]s.
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
/// * `query` -[Query] for [Map].
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
    let mut map = query.single_mut();
    loop {
        match &map.map.first() {
            None => {
                if time.elapsed_seconds() % (0.7 / speed) < time.delta_seconds() {
                    let random_num = rand::random::<usize>() % 200;
                    if random_num < 7 {
                        create_hole(None, None, &mut commands, &hole.0);
                    } else if random_num < 20 {
                        create_bar(None, None, &mut commands, &energy_bar.0);
                    } else if random_num < 21 {
                        create_rainbow(None, None, &mut commands, &rainbow.0);
                    } else if random_num < 60 {
                        if random_num % 2 == 0 {
                            create_plane_sensor(None, PlaneDir::Left, &mut commands);
                        } else {
                            create_plane_sensor(None, PlaneDir::Right, &mut commands);
                        }
                    } else if random_num < 80 {
                        create_planet(None, None, &mut commands, &planets.0);
                    } else {
                        create_basic(
                            None,
                            None,
                            &mut commands,
                            &full_choc.0,
                            &part_choc.0,
                            &egg.0,
                            &lolly.0,
                            &love.0,
                            &drink.0,
                        );
                    }
                }
                break;
            }
            Some(first) => {
                if first.time_ms <= (time.elapsed_seconds() * 1000.) as u64 {
                    match &first.enemy {
                        Enemy::HoleE => {
                            create_hole(Some(first.x as f32), None, &mut commands, &hole.0)
                        }

                        Enemy::BarE => {
                            create_bar(Some(first.x as f32), None, &mut commands, &energy_bar.0)
                        }

                        Enemy::RainbowE => {
                            create_rainbow(Some(first.x as f32), None, &mut commands, &rainbow.0)
                        }
                        Enemy::PlaneE => create_plane_sensor(None, PlaneDir::Right, &mut commands),
                        Enemy::PlanetE => {
                            create_planet(Some(first.x as f32), None, &mut commands, &planets.0)
                        }
                        Enemy::BasicE => create_basic(
                            Some(first.x as f32),
                            None,
                            &mut commands,
                            &full_choc.0,
                            &part_choc.0,
                            &egg.0,
                            &lolly.0,
                            &love.0,
                            &drink.0,
                        ),
                    };
                    map.map.pop_first();
                } else {
                    break;
                }
            }
        }
    }
}
