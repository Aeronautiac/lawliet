use std::collections::BTreeMap;

use crate::{ID, actor::Actor};

// BTreeMaps for determinism and system stability with add and remove operations
// O(logn), so operations are essentially constant time at any scale that matters
pub struct World {
    pub blackout: bool,
    pub actors: BTreeMap<ID, crate::actor::Actor>,
    pub abilities: BTreeMap<ID, crate::ability::Ability>,
    next_actor_id: ID,
}

impl World {
    pub fn new() -> Self {
        World {
            blackout: false,
            actors: BTreeMap::new(),
            abilities: BTreeMap::new(),
            next_actor_id: 0,
        }
    }

    pub fn add_actor(&mut self, actor: Actor) -> ID {
        let id = self.next_actor_id;
        self.next_actor_id += 1;
        self.actors.insert(id, actor);
        id
    }
}
