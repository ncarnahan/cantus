use std::collections::HashMap;
use std::path::Path;
use std::io::{File, FileMode, FileAccess, FilePermission};
use std::io::fs::{self, PathExtensions};
use serialize::json::{self, Json};
use uuid::Uuid;

pub fn compile_path(path: &Path, output_folder: &Path) {
    if path.is_dir() {
        let contents = fs::readdir(path).ok().unwrap();
        for entry in contents.iter() {
            if entry.is_dir() {
                compile_path(entry, output_folder);
            } else {
                compile_asset(entry, output_folder);
            }
        }
    }
    else if path.is_file() {
        compile_asset(path, output_folder);
    }
}

pub fn compile_asset(path: &Path, output_folder: &Path) {
    let ext = path.extension_str()
        .expect("No extension. Cannot determine file type.");

    let mut file = File::open(path)
        .ok().expect("Unable to open file.");

    //Output is relative to output_folder
    let mut output_path = output_folder.clone();
    output_path.push(path);
    output_path.set_extension(get_compiled_extension(ext));
    
    //Create directories + output file
    fs::mkdir_recursive(&output_path.dir_path(), FilePermission::all());
    let mut output_file = File::create(&output_path)
        .ok().expect("Unable to open output file.");

    //Compile the file based on extension
    match ext {
        "scene" => compile_scene(&mut file, &mut output_file),
        _ => { }
    }
}

fn get_compiled_extension(ext: &str) -> &'static str {
    match ext {
        "scene" => "cscene",
        _ => ""
    }
}

fn compile_scene(input: &mut Reader, output: &mut Writer) {
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

        //Write arrays to file
        output.write_le_u32(len as u32);
        for id in ids.iter() { output.write_le_u32(*id); }
        for pos in positions.iter() {
            output.write_le_f32(pos[0]);
            output.write_le_f32(pos[1]);
            output.write_le_f32(pos[2]);
        }
        for rot in rotations.iter() {
            output.write_le_f32(rot[0]);
            output.write_le_f32(rot[1]);
            output.write_le_f32(rot[2]);
            output.write_le_f32(rot[3]);
        }
        for scale in scales.iter() {
            output.write_le_f32(*scale);
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
