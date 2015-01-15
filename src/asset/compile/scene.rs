use std::collections::HashMap;
use serialize::json::{self, Json};
use uuid::Uuid;


pub fn compile_scene(input: &mut Reader, output: &mut Writer) {
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
