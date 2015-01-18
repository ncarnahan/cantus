use std::collections::HashMap;
use scene::entity::Entity;
use scene::entity_manager::EntityManager;
use cgmath::{Vector3, Quaternion};

pub struct TransformSystem {
    entities: Vec<Entity>,
    positions: Vec<Vector3<f32>>,
    rotations: Vec<Quaternion<f32>>,
    scales: Vec<f32>,
}

impl TransformSystem {
    pub fn new() -> TransformSystem {
        TransformSystem {
            entities: Vec::new(),
            positions: Vec::new(),
            rotations: Vec::new(),
            scales: Vec::new(),
        }
    }

    pub fn load(&mut self, input: &mut Reader, entity_manager: &mut EntityManager, id_map: &mut HashMap<u32, Entity>) {
        let length = input.read_le_u32().ok().unwrap() as usize;
        self.entities.reserve(length);
        self.positions.reserve(length);
        self.rotations.reserve(length);
        self.scales.reserve(length);

        for i in range(0, length) {
            let idx = input.read_le_u32().ok().unwrap();
            if id_map.contains_key(&idx) {
                self.entities.push(id_map[idx]);
            }
            else {
                let e = entity_manager.create();
                id_map.insert(idx, e);
                self.entities.push(e);
            }
        }
        for i in range(0, length) {
            self.positions.push(Vector3::new(
                input.read_le_f32().ok().unwrap(),
                input.read_le_f32().ok().unwrap(),
                input.read_le_f32().ok().unwrap()));
        }
        for i in range(0, length) {
            self.rotations.push(Quaternion::new(
                input.read_le_f32().ok().unwrap(),
                input.read_le_f32().ok().unwrap(),
                input.read_le_f32().ok().unwrap(),
                input.read_le_f32().ok().unwrap()));
        }
        for i in range(0, length) {
            self.scales.push(input.read_le_f32().ok().unwrap());
        }
    }
}



#[test]
fn test_transform_system_load() {
    let mut em = EntityManager::new();
    let mut id_map = HashMap::new();
    let mut input: Vec<u8> = vec![
        0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x80, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x40, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x3f,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x40, 0x40,
    ];

    let mut ts = TransformSystem::new();
    ts.load(&mut input.as_slice(), &mut em, &mut id_map);

    assert_eq!(ts.positions, vec![
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 4.0, 0.0),
        Vector3::new(4.0, 0.0, 0.0)]);

    assert_eq!(ts.rotations, vec![
        Quaternion::new(0.0, 0.0, 0.0, 1.0),
        Quaternion::new(0.0, 0.0, 1.0, 0.0),
        Quaternion::new(0.0, 1.0, 0.0, 0.0)]);

    assert_eq!(ts.scales, vec![1.0, 2.0, 3.0]);
}