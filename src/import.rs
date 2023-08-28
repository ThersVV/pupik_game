use crate::{
    map_layout::{Enemy, Map, SpawnEvent, Structure},
    speed::Speed,
};
use bevy::prelude::*;
use std::collections::BTreeSet;
use std::time::Duration;
use std::{collections::BTreeMap, fs::*};

///Turns on background music on repeat.
/// # Arguments
/// * `asset_server` - [AssetServer], used to load the music in.
/// * `audio` - [Audio].
pub fn import_structures() -> Option<Vec<Structure>> {
    if let Err(_) = create_dir_all("./import") {
        return None;
    };

    let Ok(paths) = read_dir("./import") else {return None;};
    let mut result = Vec::from([]);
    for path in paths {
        let mut set = BTreeSet::from([]);
        for line in read_to_string(path.unwrap().path())
            .expect("Error reading file \"export\"")
            .lines()
        {
            let line_split = line.split(' ').collect::<Vec<&str>>();
            let error_message = "Incorrect formatting of the \"export\" file";
            let x = Some(line_split[0].parse::<i32>().expect(error_message));
            let y = Some(line_split[1].parse::<i32>().expect(error_message) + 500);
            let enemy = match line_split[2] {
                "blackhole" => Enemy::HoleE,
                "rainbow" => Enemy::RainbowE,
                "energybar" => Enemy::BarE,
                "regular" => Enemy::BasicE,
                "plane" => Enemy::PlaneE,
                "planet" => Enemy::PlanetE,
                _ => Enemy::PlanetE,
            };
            set.insert(SpawnEvent { x, y, enemy });
            //map.insert(SpawnEvent{time_ms:, x, enemy})
        }
        result.push(Structure {
            spawn_chance: 999.,
            structure: set,
        })
    }
    return Some(result);
}
