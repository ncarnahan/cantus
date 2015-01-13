use std::collections::HashMap;
use std::path::Path;
use std::io::{File, FileMode, FileAccess};
use std::io::fs::{self, PathExtensions};
use serialize::json::{self, Json};
use uuid::Uuid;

pub fn compile_path(path: &Path) {
    if path.is_dir() {
        let contents = fs::readdir(path).ok().unwrap();
        for entry in contents.iter() {
            if entry.is_dir() {
                compile_path(entry);
            } else {
                compile_asset(entry);
            }
        }
    }
    else if path.is_file() {
        compile_asset(path);
    }
    else { panic!(); }
}

pub fn compile_asset(path: &Path) {
    let ext = path.extension_str()
        .expect("No extension. Cannot determine file type.");

    let mut file = File::open_mode(path, FileMode::Open, FileAccess::Read)
        .ok().expect("Unable to open file.");

    match ext {
        "scene" => compile_scene(&mut file),
        _ => { }
    }
}

fn compile_scene(input: &mut Reader) {
    let root = json::from_reader(input).ok().unwrap();

    let mut uuid_map = HashMap::new();
    let mut next_id = 0u32;

    {
        let transforms = root["transforms"].as_array().unwrap();

        //Allocate space for arrays
        let len = transforms.len();
        let mut ids = Vec::with_capacity(len);
        let mut positions = Vec::with_capacity(len);
        let mut rotations = Vec::with_capacity(len);
        let mut scales = Vec::with_capacity(len);

        //Fill arrays
        for transform in transforms.iter() {
            let uuid = Uuid::parse_str(transform["id"].as_string().unwrap()).ok().unwrap();
            let id = if uuid_map.contains_key(&uuid) {
                uuid_map[uuid]
            }
            else {
                uuid_map.insert(uuid, next_id);
                next_id += 1;
                next_id - 1
            };
            ids.push(id);

            positions.push(parse_vector3(&transform["position"]));
            rotations.push(parse_quaternion(&transform["rotation"]));
            scales.push(transform["scale"].as_f64().unwrap() as f32);
        }

        for id in ids.iter() {
            println!("{}", id);
        }
        for p in positions.iter() {
            println!("({} {} {})", p[0], p[1], p[2]);
        }
        for r in rotations.iter() {
            println!("({} {} {} {})", r[0], r[1], r[2], r[3]);
        }
        for s in scales.iter() {
            println!("{}", s);
        }
    }
}

fn parse_vector3(json: &Json) -> [f32; 3] {
    let comps: Vec<&str> = json.as_string().unwrap().split_str(" ").collect();
    assert!(comps.len() == 3);

    [
        comps[0].parse().unwrap(),
        comps[1].parse().unwrap(),
        comps[2].parse().unwrap(),
    ]
}

fn parse_quaternion(json: &Json) -> [f32; 4] {
    let comps: Vec<&str> = json.as_string().unwrap().split_str(" ").collect();
    assert!(comps.len() == 4);

    [
        comps[0].parse().unwrap(),
        comps[1].parse().unwrap(),
        comps[2].parse().unwrap(),
        comps[3].parse().unwrap(),
    ]
}
