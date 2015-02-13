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

    pub fn load(&mut self, input: &mut Reader, entity_manager: &mut EntityManager, id_map: &mut HashMap<u32, Entity>) {
        let length = input.read_le_u32().ok().unwrap() as usize;
        self.entities.reserve(length);
        self.positions.reserve(length);
        self.rotations.reserve(length);
        self.scales.reserve(length);

        for i in 0..length {
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

    /// Adds a transform component to an entity.
    ///
    /// Returns the instance.
    pub fn create(&mut self, entity: Entity) -> EntityInstance {
        let index = self.positions.len();

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
            let parent_index = self.map[entity].index;

            let mut child = self.first_children[parent_index];

            while child.is_valid() {
                let next = self.next_siblings[child.index];

                let child_entity = self.get_entity(child);
                self.destroy(child_entity);

                child = next;
            }
        }


        let instance = self.map[entity];
        let index = instance.index;

        let last_index = self.positions.len() - 1;
        let last_entity = self.entities[last_index];

        //Remove references to instance
        self.remove_instance(instance);

        //Copy last to removed
        self.move_instance(EntityInstance::new(last_index), instance);

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
        let index = instance.index;

        //Update other references to removed
        let parent_instance = self.parents[index];
        let prev_sibling = self.prev_sibling[index];
        let next_siblings = self.next_siblings[index];

        //Update the parent if we're the first child
        if parent_instance.is_valid() && self.first_children[parent_instance.index] == instance {
            self.first_children[parent_instance.index] = next_siblings;
        }
        //Update the previous sibling to point to the next
        if prev_sibling.is_valid() {
            self.next_siblings[prev_sibling.index] = next_siblings;
        }
        //Update the next sibling to point to the previous
        if next_siblings.is_valid() {
            self.prev_sibling[next_siblings.index] = prev_sibling;
        }
    }

    fn move_instance(&mut self, src_instance: EntityInstance, dst_instance: EntityInstance) {
        let src_index = src_instance.index;
        let dst_index = dst_instance.index;

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
            if parent_instance.is_valid() && self.first_children[parent_instance.index] == src_instance {
                self.first_children[parent_instance.index] = dst_instance;
            }
            //Update the previous sibling to point to the next
            if prev_sibling.is_valid() {
                self.next_siblings[prev_sibling.index] = dst_instance;
            }
            //Update the next sibling to point to the previous
            if next_siblings.is_valid() {
                self.prev_sibling[next_siblings.index] = dst_instance;
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
        self.entities[instance.index]
    }



    pub fn get_position(&self, instance: EntityInstance) -> Vector3<f32> {
        self.positions[instance.index]
    }

    pub fn set_position(&mut self, instance: EntityInstance, position: Vector3<f32>) {
        self.positions[instance.index] = position;
    }

    pub fn get_rotation(&self, instance: EntityInstance) -> Quaternion<f32> {
        self.rotations[instance.index]
    }

    pub fn set_rotation(&mut self, instance: EntityInstance, rotation: Quaternion<f32>) {
        self.rotations[instance.index] = rotation;
    }

    pub fn get_scale(&self, instance: EntityInstance) -> f32 {
        self.scales[instance.index]
    }

    pub fn set_scale(&mut self, instance: EntityInstance, scale: f32) {
        self.scales[instance.index] = scale;
    }



    pub fn get_parent(&self, instance: EntityInstance) -> EntityInstance {
        self.parents[instance.index]
    }

    pub fn set_parent(&mut self, child: EntityInstance, parent: EntityInstance) {
        //TODO: Update the old parent and siblings

        //Set the parent of the child
        self.parents[child.index] = parent;

        //Update the parent
        let old_child = self.first_children[parent.index];
        self.first_children[parent.index] = child;
        self.next_siblings[child.index] = old_child;
        if old_child.is_valid() {
            self.prev_sibling[old_child.index] = child;
        }
    }

    pub fn get_first_children(&self, instance: EntityInstance) -> EntityInstance {
        self.first_children[instance.index]
    }

    pub fn get_next_sibling(&self, instance: EntityInstance) -> EntityInstance {
        self.next_siblings[instance.index]
    }

    pub fn get_prev_sibling(&self, instance: EntityInstance) -> EntityInstance {
        self.prev_sibling[instance.index]
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