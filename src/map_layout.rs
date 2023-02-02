use crate::{
    basic::create,
    blackhole::create_hole,
    energybars::create_bar,
    homing::create_rainbow,
    plane::{create_plane_sensor, PlaneDir},
    planet::create_planet,
    EggSheet, EnergySheet, FullChocSheet, HolesSheet, KofolaSheet, LollySheet, LoveSheet,
    PartChocSheet, PlanetSheet, RainbowSheet,
};

use bevy::prelude::*;
pub struct MapPlugin;
use std::collections::BTreeSet;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_map).add_system(spawning);
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Enemy {
    HoleE,
    BarE,
    RainbowE,
    PlaneE,
    PlanetE,
    BasicE,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct SpawnEvent {
    time_ms: u64,
    x: i32,
    enemy: Enemy,
}

#[derive(Component)]
struct Map {
    map: BTreeSet<SpawnEvent>,
}

fn spawn_map(mut commands: Commands) {
    let map = commands
        .spawn(Map {
            map: BTreeSet::from([SpawnEvent {
                time_ms: 3000,
                x: 100,
                enemy: Enemy::HoleE,
            }]),
        })
        .id();
    commands.entity(map);
}

fn spawning(
    time: Res<Time>,
    mut commands: Commands,
    planets: Res<PlanetSheet>,
    hole: Res<HolesSheet>,
    bar: Res<EnergySheet>,
    rainbow: Res<RainbowSheet>,
    full_choc: Res<FullChocSheet>,
    part_choc: Res<PartChocSheet>,
    egg: Res<EggSheet>,
    lolly: Res<LollySheet>,
    love: Res<LoveSheet>,
    drink: Res<KofolaSheet>,
    mut query: Query<&mut Map, With<Map>>,
) {
    let mut map = query.single_mut();
    loop {
        match &map.map.first() {
            None => {
                if time.elapsed_seconds() % 0.7 < time.delta_seconds() {
                    let random_num = rand::random::<usize>() % 200;
                    if random_num < 10 {
                        create_hole(None, None, &mut commands, &hole.0);
                    } else if random_num < 20 {
                        create_bar(None, None, &mut commands, &bar.0);
                    } else if random_num < 21 {
                        create_rainbow(Some(0.), None, &mut commands, &rainbow.0);
                    } else if random_num < 60 {
                        if random_num % 2 == 0 {
                            create_plane_sensor(None, PlaneDir::Left, &mut commands);
                        } else {
                            create_plane_sensor(None, PlaneDir::Right, &mut commands);
                        }
                    } else if random_num < 80 {
                        create_planet(None, None, &mut commands, &planets.0);
                    } else {
                        create(
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
                /* println!(
                    "{:?}, {:?}, {:?}",
                    first.time_ms,
                    (time.elapsed_seconds() * 1000.) as u64,
                    first.enemy
                ); */
                if &first.time_ms <= &((time.elapsed_seconds() * 1000.) as u64) {
                    match &first.enemy {
                        Enemy::HoleE => {
                            create_hole(Some(first.x as f32), None, &mut commands, &hole.0)
                        }

                        Enemy::BarE => {
                            create_bar(Some(first.x as f32), None, &mut commands, &bar.0)
                        }

                        Enemy::RainbowE => {
                            create_rainbow(Some(first.x as f32), None, &mut commands, &rainbow.0)
                        }
                        Enemy::PlaneE => create_plane_sensor(None, PlaneDir::Right, &mut commands),
                        Enemy::PlanetE => {
                            create_planet(Some(first.x as f32), None, &mut commands, &planets.0)
                        }
                        Enemy::BasicE => create(
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
