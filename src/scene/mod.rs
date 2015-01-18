use std::collections::HashMap;
use scene::entity_manager::EntityManager;
use scene::transform_system::TransformSystem;

mod entity;
mod entity_instance;
mod entity_manager;
mod transform_system;


struct Scene {
    entity_manager: EntityManager,
    transform_system: TransformSystem,
}

impl Scene {
    fn load(input: &mut Reader) -> Scene {
        //When a system finds a new ID, it creates a new Entity.
        //The Entity that corresponds to an ID is tracked by this HashMap.
        let mut id_map = HashMap::new();

        let mut scene = Scene {
            entity_manager: EntityManager::new(),
            transform_system: TransformSystem::new()
        };

        scene.transform_system.load(input, &mut scene.entity_manager, &mut id_map);

        scene
    }
}
