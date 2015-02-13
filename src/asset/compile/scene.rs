use std::collections::HashMap;
use serialize::json::{self, Json};
use cgmath::{Vector3, Quaternion};
use scene::{Scene, Entity};
use uuid::Uuid;


pub fn compile_scene(input: &mut Reader, output: &mut Writer) {
    let root = json::from_reader(input).ok().unwrap();

    let entities = root["entities"].as_array().unwrap();

    //The first value written to the file is the number of entities to create.
    //When we load the file, we create all the entities at once and store them
    //in an array for easy access (since entities reference each other by ID in
    //the compiled format).
    output.write_le_u32(entities.len() as u32);

    //For now, during the compilation step, we'll keep a map of UUIDs to IDs.
    //The IDs count up starting from 0, so the map would look like this:
    //  70a44f30-e3e6-45d3-a266-8afd8652f9a0  =>  0
    //  f2c23206-83a4-40c3-aa7d-a903f4bdcbbc  =>  1
    //  65840c45-62ce-46de-870b-7ad304c882d1  =>  2
    let mut uuid_map = HashMap::new();


    //Components are grouped by entity in the intermediate format (.scene)
    //since it is easier for humans to look at. We'll manually push the data
    //to the systems and let them write it in the format that makes the most
    //sense for fast loading (usually just a direct memory dump).
    let mut scene = Scene::new();


    //First pass, create UUID map.
    for entity in entities.iter() {
        let uuid_str = entity["id"].as_string().unwrap();
        let uuid = Uuid::parse_str(uuid_str).ok().unwrap();

        //ASSUMPTION: From an empty EntityManager, sequential create() calls
        //result in Entities with sequential ids starting from 0. Should be
        //upheld by the sequential_test() unit test.
        let en = scene.entity_manager.create();
        uuid_map.insert(uuid, en.id);
    }


    //Second pass, create components.
    let mut idx = 0;
    for entity in entities.iter() {
        //Based on the ASSUMPTION above
        let en = Entity::new(idx, 0);
        idx += 1;


        let components = entity["components"].as_array().unwrap();
        for comp in components.iter() {
            //Match on component type string
            let type_ = comp["type"].as_string().unwrap();
            match type_ {
                "transform" => {
                    let ref mut sys = scene.transform_system;

                    //Create component if needed
                    let inst = if sys.exists(en) { sys.get_instance(en) }
                    else { sys.create(en) };

                    sys.set_position(inst, parse_vector3(&comp["position"]));
                    sys.set_rotation(inst, parse_quaternion(&comp["rotation"]));
                    sys.set_scale(inst, comp["scale"].as_f64().unwrap() as f32);
                }
                _ => panic!("Unknown component type.")
            }
        }
    }

    scene.save(output);
}

fn parse_vector3(json: &Json) -> Vector3<f32> {
    let comps: Vec<&str> = json.as_string().unwrap().split_str(" ").collect();
    assert!(comps.len() == 3);

    Vector3::new(
        comps[0].parse().unwrap(),
        comps[1].parse().unwrap(),
        comps[2].parse().unwrap()
    )
}

fn parse_quaternion(json: &Json) -> Quaternion<f32> {
    let comps: Vec<&str> = json.as_string().unwrap().split_str(" ").collect();
    assert!(comps.len() == 4);

    Quaternion::new(
        comps[0].parse().unwrap(),
        comps[1].parse().unwrap(),
        comps[2].parse().unwrap(),
        comps[3].parse().unwrap()
    )
}
