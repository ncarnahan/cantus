/// EntityInstance is an index into a system (that you might want to use).
///
/// # Description
///
/// Systems often store their components in an array (really a structure of
/// arrays format for speed purposes). Entities are usually mapped to array
/// indices with a hashmap. EntityInstance is just a generic offering that I
/// found myself copying and pasting.
///
/// **WARNING**: Since an EntityInstance is an index into an array, DO NOT KEEP
/// IT AROUND BETWEEN FRAMES. It exists only to prevent multiple hashmap
/// lookups. Components can be deleted/moved in between updates. Indices will
/// change. Get over it.
///
/// Systems are not required to use EntityInstance. They might not store their
/// entities in an array. They might not need separate indices. They might have
/// their own instance struct.
///
/// Simply put, if you don't want to write your system to use an
/// EntityInstance, then don't.
///
/// Systems that do use EntityInstance should wrap it to prevent users from
/// using EntityInstances that were retrieved from another system.

#[derive(Clone, Show, Eq, PartialEq)]
pub struct EntityInstance {
    pub index: usize
}

impl EntityInstance {
    pub fn new(index: usize) -> EntityInstance {
        EntityInstance { index: index }
    }

    pub fn none() -> EntityInstance {
        EntityInstance { index: ::std::usize::MAX }
    }

    pub fn is_valid(&self) -> bool {
        self.index != ::std::usize::MAX
    }
}
