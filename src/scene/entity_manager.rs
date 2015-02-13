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

//This test is used to ensure that a scene compilation assumption is upheld.
//The assumption is: From an empty EntityManager, sequential create() calls
//result in Entities with sequential ids starting from 0.
#[test]
fn sequential_test() {
    let mut manager = EntityManager::new();
    for i in 0..2*MINIMUM_FREE_INDICES {
        let en = manager.create();
        assert_eq!(en.id, i as u32);
    }
}
