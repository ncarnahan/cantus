use std::collections::{RingBuf};
use scene::entity::{Entity, ENTITY_INDEX_MASK};

pub struct EntityManager {
    generation: Vec<u8>,
    free_indices: RingBuf<u32>,
    destroyed: Vec<Entity>
}

static MINIMUM_FREE_INDICES: usize = 1024;

impl EntityManager {
    pub fn new() -> EntityManager {
        EntityManager {
            generation: Vec::new(),
            free_indices: RingBuf::new(),
            destroyed: Vec::new()
        }
    }

    pub fn alive(&self, entity: Entity) -> bool {
        self.generation[entity.index() as usize] == entity.generation()
    }

    pub fn create(&mut self) -> Entity {
        let mut index;

        if self.free_indices.len() > MINIMUM_FREE_INDICES {
            index = self.free_indices.pop_front().unwrap();
        }
        else {
            self.generation.push(0);
            index = (self.generation.len() - 1) as u32;

            assert!(index <= ENTITY_INDEX_MASK);
        }

        return Entity::new(index, self.generation[index as usize]);
    }

    pub fn destroy(&mut self, entity: Entity) {
        let index = entity.index();
        self.generation[index as usize] += 1;
        self.free_indices.push_back(index);

        //Keep track of the destroyed entities
        self.destroyed.push(entity);
    }

    pub fn poll_destroyed(&self) -> &[Entity] {
        self.destroyed.as_slice()
    }

    pub fn clear_destroyed(&mut self) {
        self.destroyed.clear();
    }
}
