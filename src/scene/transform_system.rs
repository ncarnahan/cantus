use std::collections::HashMap;
use scene::entity::Entity;
use scene::entity_manager::EntityManager;
use scene::entity_instance::EntityInstance;
use cgmath::{Vector3, Quaternion};

pub struct TransformSystem {
    map: HashMap<Entity, EntityInstance>,

    entities: Vec<Entity>,

    positions: Vec<Vector3<f32>>,
    rotations: Vec<Quaternion<f32>>,
    scales: Vec<f32>,

    parents: Vec<EntityInstance>,
    first_children: Vec<EntityInstance>,
    next_siblings: Vec<EntityInstance>,
    prev_sibling: Vec<EntityInstance>,
}

impl TransformSystem {
    pub fn new() -> TransformSystem {
        TransformSystem {
            map: HashMap::new(),

            entities: Vec::new(),

            positions: Vec::new(),
            rotations: Vec::new(),
            scales: Vec::new(),

            parents: Vec::new(),
            first_children: Vec::new(),
            next_siblings: Vec::new(),
            prev_sibling: Vec::new(),
        }
    }

    pub fn load(&mut self, input: &mut Reader, id_map: &[Entity]) {
        let length = input.read_le_u32().ok().unwrap() as usize;
        self.entities.reserve(length);
        self.positions.reserve(length);
        self.rotations.reserve(length);
        self.scales.reserve(length);

        for i in 0..length as u32 {
            let idx = input.read_le_u32().ok().unwrap();
            let en = id_map[idx as usize];
            self.entities.push(en);
            self.map.insert(en, EntityInstance::new(i));
        }
        for i in 0..length {
            self.positions.push(Vector3::new(
                input.read_le_f32().ok().unwrap(),
                input.read_le_f32().ok().unwrap(),
                input.read_le_f32().ok().unwrap()));
        }
        for i in 0..length {
            self.rotations.push(Quaternion::new(
                input.read_le_f32().ok().unwrap(),
                input.read_le_f32().ok().unwrap(),
                input.read_le_f32().ok().unwrap(),
                input.read_le_f32().ok().unwrap()));
        }
        for i in 0..length {
            self.scales.push(input.read_le_f32().ok().unwrap());
        }
    }

    pub fn save(&self, output: &mut Writer) {
        output.write_le_u32(self.entities.len() as u32);
        for en in &self.entities { output.write_le_u32(en.id); }
        for pos in &self.positions {
            output.write_le_f32(pos.x);
            output.write_le_f32(pos.y);
            output.write_le_f32(pos.z);
        }
        for rot in &self.rotations {
            output.write_le_f32(rot.s);
            output.write_le_f32(rot.v.x);
            output.write_le_f32(rot.v.y);
            output.write_le_f32(rot.v.z);
        }
        for scale in &self.scales {
            output.write_le_f32(*scale);
        }
    }

    pub fn exists(&self, entity: Entity) -> bool {
        self.map.contains_key(&entity)
    }

    /// Adds a transform component to an entity.
    ///
    /// Returns the instance.
    pub fn create(&mut self, entity: Entity) -> EntityInstance {
        let index = self.positions.len() as u32;

        //Add component values
        self.entities.push(entity);

        self.positions.push(Vector3::new(0.0, 0.0, 0.0));
        self.rotations.push(Quaternion::identity());
        self.scales.push(0.0);

        self.parents.push(EntityInstance::none());
        self.first_children.push(EntityInstance::none());
        self.next_siblings.push(EntityInstance::none());
        self.prev_sibling.push(EntityInstance::none());

        let instance = EntityInstance::new(index);
        self.map.insert(entity, instance);

        instance
    }

    pub fn destroy(&mut self, entity: Entity) {
        //Remove children
        {
            let parent_index = self.map[entity].idx();

            let mut child = self.first_children[parent_index];

            while child.is_valid() {
                let next = self.next_siblings[child.idx()];

                let child_entity = self.get_entity(child);
                self.destroy(child_entity);

                child = next;
            }
        }


        let instance = self.map[entity];
        let index = instance.idx();

        let last_index = self.positions.len() - 1;
        let last_entity = self.entities[last_index];

        //Remove references to instance
        self.remove_instance(instance);

        //Copy last to removed
        self.move_instance(EntityInstance::new(last_index as u32), instance);

        //Remove last
        self.entities.pop();
        self.parents.pop();
        self.first_children.pop();
        self.next_siblings.pop();
        self.prev_sibling.pop();
        self.positions.pop();

        //Update keys in the map
        self.map.insert(last_entity, instance);
        self.map.remove(&entity);
    }

    fn remove_instance(&mut self, instance: EntityInstance) {
        let index = instance.idx();

        //Update other references to removed
        let parent_instance = self.parents[index];
        let prev_sibling = self.prev_sibling[index];
        let next_siblings = self.next_siblings[index];

        //Update the parent if we're the first child
        if parent_instance.is_valid() && self.first_children[parent_instance.idx()] == instance {
            self.first_children[parent_instance.idx()] = next_siblings;
        }
        //Update the previous sibling to point to the next
        if prev_sibling.is_valid() {
            self.next_siblings[prev_sibling.idx()] = next_siblings;
        }
        //Update the next sibling to point to the previous
        if next_siblings.is_valid() {
            self.prev_sibling[next_siblings.idx()] = prev_sibling;
        }
    }

    fn move_instance(&mut self, src_instance: EntityInstance, dst_instance: EntityInstance) {
        let src_index = src_instance.idx();
        let dst_index = dst_instance.idx();

        //Copy source to destination
        {
            self.entities[dst_index] = self.entities[src_index];
            self.parents[dst_index] = self.parents[src_index];
            self.first_children[dst_index] = self.first_children[src_index];
            self.next_siblings[dst_index] = self.next_siblings[src_index];
            self.prev_sibling[dst_index] = self.prev_sibling[src_index];
            self.positions[dst_index] = self.positions[src_index];
        }

        //Update other references to source
        {
            let parent_instance = self.parents[src_index];
            let prev_sibling = self.prev_sibling[src_index];
            let next_siblings = self.next_siblings[src_index];

            //Update the parent if we're the first child
            if parent_instance.is_valid() && self.first_children[parent_instance.idx()] == src_instance {
                self.first_children[parent_instance.idx()] = dst_instance;
            }
            //Update the previous sibling to point to the next
            if prev_sibling.is_valid() {
                self.next_siblings[prev_sibling.idx()] = dst_instance;
            }
            //Update the next sibling to point to the previous
            if next_siblings.is_valid() {
                self.prev_sibling[next_siblings.idx()] = dst_instance;
            }
        }
    }

    pub fn handle_destroyed(&mut self, entity: &[Entity]) {
        for entity in entity.iter() {
            if self.map.contains_key(entity) {
                self.destroy(*entity);
            }
        }
    }

    pub fn count(&self) -> usize {
        self.entities.len()
    }



    pub fn get_instance(&self, entity: Entity) -> EntityInstance {
        if let Some(instance) = self.map.get(&entity) {
            *instance
        }
        else {
            EntityInstance::none()
        }
    }

    pub fn get_entity(&self, instance: EntityInstance) -> Entity {
        self.entities[instance.idx()]
    }



    pub fn get_position(&self, instance: EntityInstance) -> Vector3<f32> {
        self.positions[instance.idx()]
    }

    pub fn set_position(&mut self, instance: EntityInstance, position: Vector3<f32>) {
        self.positions[instance.idx()] = position;
    }

    pub fn get_rotation(&self, instance: EntityInstance) -> Quaternion<f32> {
        self.rotations[instance.idx()]
    }

    pub fn set_rotation(&mut self, instance: EntityInstance, rotation: Quaternion<f32>) {
        self.rotations[instance.idx()] = rotation;
    }

    pub fn get_scale(&self, instance: EntityInstance) -> f32 {
        self.scales[instance.idx()]
    }

    pub fn set_scale(&mut self, instance: EntityInstance, scale: f32) {
        self.scales[instance.idx()] = scale;
    }



    pub fn get_parent(&self, instance: EntityInstance) -> EntityInstance {
        self.parents[instance.idx()]
    }

    pub fn set_parent(&mut self, child: EntityInstance, parent: EntityInstance) {
        //TODO: Update the old parent and siblings

        //Set the parent of the child
        self.parents[child.idx()] = parent;

        //Update the parent
        let old_child = self.first_children[parent.idx()];
        self.first_children[parent.idx()] = child;
        self.next_siblings[child.idx()] = old_child;
        if old_child.is_valid() {
            self.prev_sibling[old_child.idx()] = child;
        }
    }

    pub fn get_first_children(&self, instance: EntityInstance) -> EntityInstance {
        self.first_children[instance.idx()]
    }

    pub fn get_next_sibling(&self, instance: EntityInstance) -> EntityInstance {
        self.next_siblings[instance.idx()]
    }

    pub fn get_prev_sibling(&self, instance: EntityInstance) -> EntityInstance {
        self.prev_sibling[instance.idx()]
    }
}



#[test]
fn test_transform_system_load() {
    let entities = vec![
        Entity::new(0, 0),
        Entity::new(1, 0),
        Entity::new(2, 0),
    ];
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
    ts.load(&mut input.as_slice(), &entities[..]);

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

#[test]
fn test_exists() {
    let mut tr_system = TransformSystem::new();
    let en1 = Entity::new(0, 0);
    let en2 = Entity::new(100, 50);
    assert_eq!(tr_system.exists(en1), false);
    assert_eq!(tr_system.exists(en2), false);
    
    tr_system.create(en1);
    tr_system.create(en2);
    assert_eq!(tr_system.exists(en1), true);
    assert_eq!(tr_system.exists(en2), true);
}
