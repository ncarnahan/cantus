use std::collections::HashMap;
use scene::entity::Entity;
use cgmath::{Vector3, Quaternion};

pub struct TransformSystem {
    entities: Vec<Entity>,
    positions: Vec<Vector3<f32>>,
    rotations: Vec<Quaternion<f32>>,
    scales: Vec<f32>,
}

impl TransformSystem {
    pub fn load(input: &mut Reader, id_map: &mut HashMap<u32, Entity>) -> TransformSystem {
        let length = input.read_le_u32().ok().unwrap();

        let mut ts = TransformSystem {
            entities: Vec::with_capacity(length as usize),
            positions: Vec::with_capacity(length as usize),
            rotations: Vec::with_capacity(length as usize),
            scales: Vec::with_capacity(length as usize),
        };

        for i in range(0, length) {
            let idx = input.read_le_u32().ok().unwrap();
            ts.entities.push(id_map[idx].clone());
        }
        for i in range(0, length) {
            ts.positions.push(Vector3::new(
                input.read_le_f32().ok().unwrap(),
                input.read_le_f32().ok().unwrap(),
                input.read_le_f32().ok().unwrap()));
        }
        for i in range(0, length) {
            ts.rotations.push(Quaternion::new(
                input.read_le_f32().ok().unwrap(),
                input.read_le_f32().ok().unwrap(),
                input.read_le_f32().ok().unwrap(),
                input.read_le_f32().ok().unwrap()));
        }
        for i in range(0, length) {
            ts.scales.push(input.read_le_f32().ok().unwrap());
        }

        ts
    }
}
