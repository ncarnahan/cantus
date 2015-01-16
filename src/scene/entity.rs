#[derive(Show, Eq, PartialEq, Clone, Hash)]
pub struct Entity {
    pub id: u32
}

pub const ENTITY_INDEX_BITS: u32 = 24;
pub const ENTITY_INDEX_MASK: u32 = (1 << ENTITY_INDEX_BITS) - 1;

//If this is increased beyond 8 bits, you need to update the EntityManager as well
pub const ENTITY_GENERATION_BITS: u32 = 8;
pub const ENTITY_GENERATION_MASK: u32 = (1 << ENTITY_GENERATION_BITS) - 1;

impl Entity {
    pub fn new(index: u32, generation: u8) -> Entity {
        assert!(index <= ENTITY_INDEX_MASK);
        assert!(generation <= ENTITY_GENERATION_MASK as u8);

        let id = ((generation as u32) << ENTITY_INDEX_BITS) | index;

        //MAX can be used by systems as a fake entity
        assert!(id != ::std::u32::MAX);

        Entity {
            id: id
        }
    }

    pub fn index(self) -> u32 {
        return self.id & ENTITY_INDEX_MASK;
    }

    pub fn generation(self) -> u8 {
        return ((self.id >> ENTITY_INDEX_BITS) & ENTITY_GENERATION_MASK) as u8;
    }
}
