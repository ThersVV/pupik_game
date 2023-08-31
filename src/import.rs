use crate::map_layout::{Enemy, SpawnEvent, Structure};
use std::fs::*;

///Turns on background music on repeat.
/// # Arguments
/// * `asvec_server` - [AsvecServer], used to load the music in.
/// * `audio` - [Audio].
pub fn import_structures() -> Option<Vec<Structure>> {
    if let Err(_) = create_dir_all("./structures") {
        return None;
    };

    let Ok(paths) = read_dir("./structures") else {return None;};
    let mut result = Vec::from([]);
    for path in paths {
        let mut vec = Vec::from([]);
        let mut weight = 1.;
        let mut first = true;
        let mut min_y = i32::MAX;
        for line in read_to_string(path.unwrap().path())
            .expect("Error reading file \"structures\"")
            .lines()
        {
            if first {
                first = false;
                let line_split = line.split(' ').collect::<Vec<&str>>();
                weight = line_split[0].parse::<f64>().unwrap();
                continue;
            }
            let line_split = line.split(' ').collect::<Vec<&str>>();
            let error_message = "Incorrect formatting of the \"structures\" file";
            let x = Some(line_split[0].parse::<i32>().expect(error_message));
            let y = Some(line_split[1].parse::<i32>().expect(error_message));
            if y.unwrap() < min_y {
                min_y = y.unwrap();
            }
            let enemy = match line_split[2] {
                "blackhole" => Enemy::HoleE,
                "rainbow" => Enemy::RainbowE,
                "energybar" => Enemy::BarE,
                "regular" => Enemy::BasicE,
                "plane" => Enemy::PlaneE,
                "planet" => Enemy::PlanetE,
                _ => Enemy::PlanetE,
            };
            vec.push(SpawnEvent {
                x,
                y: y.map(|y| y + 600),
                enemy,
            });
        }
        for object in vec.iter_mut() {
            object.y = Some(object.y.unwrap() - min_y);
        }
        result.push(Structure {
            spawn_chance: weight,
            structure: vec,
        })
    }
    return Some(result);
}
