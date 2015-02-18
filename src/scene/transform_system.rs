use std::collections::HashMap;
use scene::entity::Entity;
use scene::entity_manager::EntityManager;
use scene::entity_instance::EntityInstance;
use cgmath::{Vector3, Quaternion};

pub struct TransformSystem {
    map: HashMap<Entity, EntityInstance>,

    entities: Vec<Entity>,

    local_positions: Vec<Vector3<f32>>,
    local_rotations: Vec<Quaternion<f32>>,
    local_scales: Vec<f32>,

    world_positions: Vec<Vector3<f32>>,
    world_rotations: Vec<Quaternion<f32>>,
    world_scales: Vec<f32>,

    parents: Vec<EntityInstance>,
    first_children: Vec<EntityInstance>,
    next_siblings: Vec<EntityInstance>,
    prev_siblings: Vec<EntityInstance>,
}

impl TransformSystem {
    pub fn new() -> TransformSystem {
        TransformSystem {
            map: HashMap::new(),

            entities: Vec::new(),

            local_positions: Vec::new(),
            local_rotations: Vec::new(),
            local_scales: Vec::new(),

            world_positions: Vec::new(),
            world_rotations: Vec::new(),
            world_scales: Vec::new(),

            parents: Vec::new(),
            first_children: Vec::new(),
            next_siblings: Vec::new(),
            prev_siblings: Vec::new(),
        }
    }

    pub fn load(&mut self, input: &mut Reader, id_map: &[Entity]) {
        //Reserve space
        let length = input.read_le_u32().ok().unwrap() as usize;
        self.entities.reserve(length);
        self.local_positions.reserve(length);
        self.local_rotations.reserve(length);
        self.local_scales.reserve(length);

        self.world_positions.reserve(length);
        self.world_rotations.reserve(length);
        self.world_scales.reserve(length);

        self.parents.reserve(length);
        self.first_children.reserve(length);
        self.next_siblings.reserve(length);
        self.prev_siblings.reserve(length);


        //Read values
        for i in 0..length as u32 {
            let idx = input.read_le_u32().ok().unwrap();
            let en = id_map[idx as usize];
            self.entities.push(en);
            self.map.insert(en, EntityInstance::new(i));
        }

        for i in 0..length {
            self.local_positions.push(Vector3::new(
                input.read_le_f32().ok().unwrap(),
                input.read_le_f32().ok().unwrap(),
                input.read_le_f32().ok().unwrap()));
        }
        for i in 0..length {
            self.local_rotations.push(Quaternion::new(
                input.read_le_f32().ok().unwrap(),
                input.read_le_f32().ok().unwrap(),
                input.read_le_f32().ok().unwrap(),
                input.read_le_f32().ok().unwrap()));
        }
        for i in 0..length {
            self.local_scales.push(input.read_le_f32().ok().unwrap());
        }

        for i in 0..length {
            self.world_positions.push(Vector3::new(
                input.read_le_f32().ok().unwrap(),
                input.read_le_f32().ok().unwrap(),
                input.read_le_f32().ok().unwrap()));
        }
        for i in 0..length {
            self.world_rotations.push(Quaternion::new(
                input.read_le_f32().ok().unwrap(),
                input.read_le_f32().ok().unwrap(),
                input.read_le_f32().ok().unwrap(),
                input.read_le_f32().ok().unwrap()));
        }
        for i in 0..length {
            self.world_scales.push(input.read_le_f32().ok().unwrap());
        }

        for i in 0..length {
            let idx = input.read_le_u32().ok().unwrap();
            self.parents.push(EntityInstance::new(idx));
        }
        for i in 0..length {
            let idx = input.read_le_u32().ok().unwrap();
            self.first_children.push(EntityInstance::new(idx));
        }
        for i in 0..length {
            let idx = input.read_le_u32().ok().unwrap();
            self.next_siblings.push(EntityInstance::new(idx));
        }
        for i in 0..length {
            let idx = input.read_le_u32().ok().unwrap();
            self.prev_siblings.push(EntityInstance::new(idx));
        }
    }

    pub fn save(&self, output: &mut Writer) {
        output.write_le_u32(self.entities.len() as u32);
        for en in &self.entities { output.write_le_u32(en.id); }

        for pos in &self.local_positions {
            output.write_le_f32(pos.x);
            output.write_le_f32(pos.y);
            output.write_le_f32(pos.z);
        }
        for rot in &self.local_rotations {
            output.write_le_f32(rot.s);
            output.write_le_f32(rot.v.x);
            output.write_le_f32(rot.v.y);
            output.write_le_f32(rot.v.z);
        }
        for scale in &self.local_scales {
            output.write_le_f32(*scale);
        }

        for pos in &self.world_positions {
            output.write_le_f32(pos.x);
            output.write_le_f32(pos.y);
            output.write_le_f32(pos.z);
        }
        for rot in &self.world_rotations {
            output.write_le_f32(rot.s);
            output.write_le_f32(rot.v.x);
            output.write_le_f32(rot.v.y);
            output.write_le_f32(rot.v.z);
        }
        for scale in &self.world_scales {
            output.write_le_f32(*scale);
        }
        
        for id in &self.parents { output.write_le_u32(id.index); }
        for id in &self.first_children { output.write_le_u32(id.index); }
        for id in &self.next_siblings { output.write_le_u32(id.index); }
        for id in &self.prev_siblings { output.write_le_u32(id.index); }
    }

    pub fn exists(&self, entity: Entity) -> bool {
        self.map.contains_key(&entity)
    }

    /// Adds a transform component to an entity.
    ///
    /// Returns the instance.
    pub fn create(&mut self, entity: Entity) -> EntityInstance {
        let index = self.local_positions.len() as u32;

        //Add component values
        self.entities.push(entity);

        self.local_positions.push(Vector3::new(0.0, 0.0, 0.0));
        self.local_rotations.push(Quaternion::identity());
        self.local_scales.push(0.0);

        self.world_positions.push(Vector3::new(0.0, 0.0, 0.0));
        self.world_rotations.push(Quaternion::identity());
        self.world_scales.push(0.0);

        self.parents.push(EntityInstance::none());
        self.first_children.push(EntityInstance::none());
        self.next_siblings.push(EntityInstance::none());
        self.prev_siblings.push(EntityInstance::none());

        let instance = EntityInstance::new(index);
        self.map.insert(entity, instance);

        instance
    }

    pub fn create_or_get_instance(&mut self, en: Entity) -> EntityInstance {
        if self.exists(en) { self.get_instance(en) }
        else { self.create(en) }
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

        let last_index = self.local_positions.len() - 1;
        let last_entity = self.entities[last_index];

        //Remove references to instance
        self.remove_instance(instance);

        //Copy last to removed
        self.move_instance(EntityInstance::new(last_index as u32), instance);

        //Remove last
        self.entities.pop();
        self.local_positions.pop();
        self.local_rotations.pop();
        self.local_scales.pop();
        self.world_positions.pop();
        self.world_rotations.pop();
        self.world_scales.pop();
        self.parents.pop();
        self.first_children.pop();
        self.next_siblings.pop();
        self.prev_siblings.pop();

        //Update keys in the map
        self.map.insert(last_entity, instance);
        self.map.remove(&entity);
    }

    fn remove_instance(&mut self, instance: EntityInstance) {
        let index = instance.idx();

        //Update other references to removed
        let parent_instance = self.parents[index];
        let prev_sibling = self.prev_siblings[index];
        let next_sibling = self.next_siblings[index];

        //Update the parent if we're the first child
        if parent_instance.is_valid() && self.first_children[parent_instance.idx()] == instance {
            self.first_children[parent_instance.idx()] = next_sibling;
        }
        //Update the previous sibling to point to the next
        if prev_sibling.is_valid() {
            self.next_siblings[prev_sibling.idx()] = next_sibling;
        }
        //Update the next sibling to point to the previous
        if next_sibling.is_valid() {
            self.prev_siblings[next_sibling.idx()] = prev_sibling;
        }
    }

    fn move_instance(&mut self, src_instance: EntityInstance, dst_instance: EntityInstance) {
        let src_index = src_instance.idx();
        let dst_index = dst_instance.idx();

        //Copy source to destination
        {
            self.entities[dst_index] = self.entities[src_index];
            self.parents[dst_index] = self.parents[src_index];
            self.local_positions[dst_index] = self.local_positions[src_index];
            self.local_rotations[dst_index] = self.local_rotations[src_index];
            self.local_scales[dst_index] = self.local_scales[src_index];
            self.world_positions[dst_index] = self.world_positions[src_index];
            self.world_rotations[dst_index] = self.world_rotations[src_index];
            self.world_scales[dst_index] = self.world_scales[src_index];
            self.first_children[dst_index] = self.first_children[src_index];
            self.next_siblings[dst_index] = self.next_siblings[src_index];
            self.prev_siblings[dst_index] = self.prev_siblings[src_index];
        }

        //Update other references to source
        {
            let parent_instance = self.parents[src_index];
            let prev_sibling = self.prev_siblings[src_index];
            let next_sibling = self.next_siblings[src_index];

            //Update the parent if we're the first child
            if parent_instance.is_valid() && self.first_children[parent_instance.idx()] == src_instance {
                self.first_children[parent_instance.idx()] = dst_instance;
            }
            //Update the previous sibling to point to the next
            if prev_sibling.is_valid() {
                self.next_siblings[prev_sibling.idx()] = dst_instance;
            }
            //Update the next sibling to point to the previous
            if next_sibling.is_valid() {
                self.prev_siblings[next_sibling.idx()] = dst_instance;
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

    fn get_entity(&self, instance: EntityInstance) -> Entity {
        self.entities[instance.idx()]
    }



    pub fn get_local_position(&self, instance: EntityInstance) -> Vector3<f32> {
        self.local_positions[instance.idx()]
    }

    pub fn set_local_position(&mut self, instance: EntityInstance, position: Vector3<f32>) {
        //TODO: Update world position + children
        self.local_positions[instance.idx()] = position;
    }

    pub fn get_local_rotation(&self, instance: EntityInstance) -> Quaternion<f32> {
        self.local_rotations[instance.idx()]
    }

    pub fn set_local_rotation(&mut self, instance: EntityInstance, rotation: Quaternion<f32>) {
        //TODO: Update world rotation + children
        self.local_rotations[instance.idx()] = rotation;
    }

    pub fn get_local_scale(&self, instance: EntityInstance) -> f32 {
        self.local_scales[instance.idx()]
    }

    pub fn set_local_scale(&mut self, instance: EntityInstance, scale: f32) {
        //TODO: Update world scale + children
        self.local_scales[instance.idx()] = scale;
    }



    pub fn get_world_position(&self, instance: EntityInstance) -> Vector3<f32> {
        self.world_positions[instance.idx()]
    }

    pub fn set_world_position(&mut self, instance: EntityInstance, position: Vector3<f32>) {
        //TODO: Update local position + children
        self.world_positions[instance.idx()] = position;
    }

    pub fn get_world_rotation(&self, instance: EntityInstance) -> Quaternion<f32> {
        self.world_rotations[instance.idx()]
    }

    pub fn set_world_rotation(&mut self, instance: EntityInstance, rotation: Quaternion<f32>) {
        //TODO: Update local rotation + children
        self.world_rotations[instance.idx()] = rotation;
    }

    pub fn get_world_scale(&self, instance: EntityInstance) -> f32 {
        self.world_scales[instance.idx()]
    }

    pub fn set_world_scale(&mut self, instance: EntityInstance, scale: f32) {
        //TODO: Update local scale + children
        self.world_scales[instance.idx()] = scale;
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
            self.prev_siblings[old_child.idx()] = child;
        }
    }

    pub fn get_first_child(&self, instance: EntityInstance) -> EntityInstance {
        self.first_children[instance.idx()]
    }

    pub fn get_next_sibling(&self, instance: EntityInstance) -> EntityInstance {
        self.next_siblings[instance.idx()]
    }

    pub fn get_prev_sibling(&self, instance: EntityInstance) -> EntityInstance {
        self.prev_siblings[instance.idx()]
    }



    pub fn iter_children<'a>(&'a self, parent: EntityInstance) -> ChildIterator<'a> {
        ChildIterator {
            next_siblings: &self.next_siblings,
            current: self.first_children[parent.idx()],
        }
    }
}



pub struct ChildIterator<'a> {
    next_siblings: &'a Vec<EntityInstance>,
    current: EntityInstance,
}

impl<'a> Iterator for ChildIterator<'a> {
    type Item = EntityInstance;
    fn next(&mut self) -> Option<EntityInstance> {
        if self.current.is_valid() {
            let output = self.current;

            //Advance to next sibling
            self.current = self.next_siblings[output.idx()];

            Some(output)
        }
        else {
            None
        }
    }
}

#[test]
fn iter_children_test() {
    let mut em = EntityManager::new();
    let mut tr = TransformSystem::new();

    let e1 = em.create();
    let e2 = em.create();
    let e3 = em.create();
    let i1 = tr.create(e1);
    let i2 = tr.create(e2);
    let i3 = tr.create(e3);
    tr.set_parent(i1, i2);
    tr.set_parent(i3, i2);

    let children: Vec<EntityInstance> = tr.iter_children(i2).collect();
    assert_eq!(children, vec![i3, i1]);
    let children: Vec<EntityInstance> = tr.iter_children(i1).collect();
    assert_eq!(children, Vec::new());
}
