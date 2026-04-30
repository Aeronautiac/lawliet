use std::{
    collections::{BTreeMap, BTreeSet, btree_map::Entry},
    rc::Rc,
};

#[derive(Debug)]
pub enum WorldError {
    DuplicateName,
}

use crate::{
    ID,
    ability::Ability,
    actor::{Actor, ActorType, Player},
    config::role::Role,
    notebook::Notebook,
    passive::Passive,
};

#[derive(Debug)]
pub struct World {
    pub blackout: bool,
    pub actors: BTreeMap<ID, Actor>,
    pub player_names: BTreeMap<Rc<str>, ID>, // a map of true names to actor ids
    pub abilities: BTreeMap<ID, Ability>,
    pub notebooks: BTreeMap<ID, Notebook>,
    pub passives: BTreeMap<ID, Passive>,
    next_actor_id: ID,
    next_notebook_id: ID,
    next_ability_id: ID,
    next_passive_id: ID,
}

impl World {
    pub fn new() -> Self {
        World {
            blackout: false,
            actors: BTreeMap::new(),
            abilities: BTreeMap::new(),
            notebooks: BTreeMap::new(),
            player_names: BTreeMap::new(),
            passives: BTreeMap::new(),
            next_actor_id: 0,
            next_notebook_id: 0,
            next_ability_id: 0,
            next_passive_id: 0,
        }
    }

    pub fn add_actor(&mut self, actor: Actor) -> ID {
        let id = self.next_actor_id;
        self.next_actor_id += 1;
        self.actors.insert(id, actor);
        id
    }

    pub fn get_actor(&self, id: ID) -> Option<&Actor> {
        self.actors.get(&id)
    }

    pub fn get_actor_mut(&mut self, id: ID) -> Option<&mut Actor> {
        self.actors.get_mut(&id)
    }

    pub fn get_player_mut(&mut self, id: ID) -> Option<&mut Player> {
        if let Some(actor) = self.actors.get_mut(&id) {
            if let ActorType::Player(player) = &mut actor.actor_type {
                Some(player)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_player(&self, id: ID) -> Option<&Player> {
        if let Some(actor) = self.actors.get(&id) {
            if let ActorType::Player(player) = &actor.actor_type {
                Some(player)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_player_id_by_name(&self, name: &str) -> Option<ID> {
        self.player_names.get(name.to_lowercase().as_str()).copied()
    }

    pub fn add_player(&mut self, true_name: &str, role: Role) -> Result<ID, WorldError> {
        let id = self.add_actor(Actor::new_player(&true_name.to_lowercase(), role));
        let name = self.get_player_mut(id).unwrap().true_name.clone();
        match self.player_names.entry(name) {
            Entry::Vacant(e) => {
                e.insert(id);
                Ok(id)
            }
            Entry::Occupied(_) => Err(WorldError::DuplicateName),
        }
    }

    pub fn add_notebook(&mut self, fake: bool) -> ID {
        let id = self.next_notebook_id;
        self.next_notebook_id += 1;
        self.notebooks.insert(id, Notebook::new(fake));
        id
    }

    pub fn get_notebook_mut(&mut self, id: ID) -> Option<&mut Notebook> {
        self.notebooks.get_mut(&id)
    }

    pub fn get_notebook(&self, id: ID) -> Option<&Notebook> {
        self.notebooks.get(&id)
    }

    pub fn add_ability(&mut self, ability: Ability) -> ID {
        let id = self.next_ability_id;
        self.next_ability_id += 1;
        self.abilities.insert(id, ability);
        id
    }

    /// be careful that there are no dangling ids
    pub fn remove_abiliy(&mut self, id: ID) {
        self.abilities.remove(&id);
    }

    pub fn get_ability(&self, id: ID) -> Option<&Ability> {
        self.abilities.get(&id)
    }

    pub fn get_ability_mut(&mut self, id: ID) -> Option<&mut Ability> {
        self.abilities.get_mut(&id)
    }

    pub fn add_passive(&mut self, passive: Passive) -> ID {
        let id = self.next_ability_id;
        self.next_passive_id += 1;
        self.passives.insert(id, passive);
        id
    }

    /// be careful that there are no dangling ids
    pub fn remove_passive(&mut self, id: ID) {
        self.passives.remove(&id);
    }

    pub fn get_passive(&self, id: ID) -> Option<&Passive> {
        self.passives.get(&id)
    }

    pub fn get_passive_mut(&mut self, id: ID) -> Option<&mut Passive> {
        self.passives.get_mut(&id)
    }
}
