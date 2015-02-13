use std::collections::HashMap;

pub use scene::entity::Entity;
pub use scene::entity_manager::EntityManager;
pub use scene::transform_system::TransformSystem;

mod entity;
mod entity_instance;
mod entity_manager;
mod transform_system;


pub struct Scene {
    pub entity_manager: EntityManager,
    pub transform_system: TransformSystem,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            entity_manager: EntityManager::new(),
            transform_system: TransformSystem::new()
        }
    }

    pub fn load(&mut self, input: &mut Reader) {
        //Create all the entities we need
        let entity_count = input.read_le_u32().ok().unwrap();
        let mut entities = Vec::with_capacity(entity_count as usize);
        for i in 0..entity_count {
            entities.push(self.entity_manager.create());
        }

        //Load each system
        self.transform_system.load(input, &entities);
    }

    pub fn save(&self, output: &mut Writer) {
        //Save each system
        self.transform_system.save(output);
    }
}
