use std::collections::HashMap;
use scene::transform_system::TransformSystem;

mod entity;
mod entity_instance;
mod transform_system;


struct Scene {
    transform_system: TransformSystem,
}

impl Scene {
    fn load(input: &mut Reader) -> Scene {
        //When a system finds a new ID, it creates a new Entity.
        //The Entity that corresponds to an ID is tracked by this HashMap.
        let mut id_map = HashMap::new();

        // let scene = unsafe { ::std::mem::uninitialized() };

        // scene.transform_system = TransformSystem::load(input, &mut id_map);

        // scene
        Scene {
            transform_system: TransformSystem::load(input, &mut id_map)
        }
    }
}
