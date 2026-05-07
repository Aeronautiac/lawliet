use std::{
    collections::{BTreeMap, btree_map::Entry},
    rc::Rc,
};

#[derive(Debug)]
pub enum WorldError {
    DuplicateName,
}

use indexmap::IndexMap;

use crate::{
    ID,
    ability::Ability,
    actor::{Actor, ActorType, Organization, Player},
    chargepool::ChargePool,
    config::{role::Role, world::WorldChargePoolName},
    notebook::Notebook,
    passive::Passive,
    poll::Poll,
};

#[derive(Debug)]
pub struct World {
    pub blackout: bool,
    pub actors: IndexMap<ID, Actor>,
    pub player_names: BTreeMap<Rc<str>, ID>, // a map of true names to actor ids
    pub abilities: IndexMap<ID, Ability>,
    pub notebooks: IndexMap<ID, Notebook>,
    pub passives: IndexMap<ID, Passive>,
    pub charge_pools: IndexMap<ID, ChargePool>,
    pub pool_map: IndexMap<WorldChargePoolName, ID>, // things like the world prosecution pool
    pub polls: IndexMap<ID, Poll>,
    next_charge_pool_id: ID,
    next_actor_id: ID,
    next_notebook_id: ID,
    next_ability_id: ID,
    next_passive_id: ID,
    next_poll_id: ID,
}

impl World {
    pub fn new() -> Self {
        World {
            blackout: false,
            actors: IndexMap::new(),
            abilities: IndexMap::new(),
            notebooks: IndexMap::new(),
            player_names: BTreeMap::new(),
            passives: IndexMap::new(),
            charge_pools: IndexMap::new(),
            pool_map: IndexMap::new(),
            polls: IndexMap::new(),
            next_charge_pool_id: 0,
            next_actor_id: 0,
            next_notebook_id: 0,
            next_ability_id: 0,
            next_passive_id: 0,
            next_poll_id: 0,
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
    pub fn remove_ability(&mut self, id: ID) {
        self.abilities.swap_remove(&id);
    }

    pub fn get_ability(&self, id: ID) -> Option<&Ability> {
        self.abilities.get(&id)
    }

    pub fn get_ability_mut(&mut self, id: ID) -> Option<&mut Ability> {
        self.abilities.get_mut(&id)
    }

    pub fn add_passive(&mut self, passive: Passive) -> ID {
        let id = self.next_passive_id;
        self.next_passive_id += 1;
        self.passives.insert(id, passive);
        id
    }

    /// be careful that there are no dangling ids
    pub fn remove_passive(&mut self, id: ID) {
        self.passives.swap_remove(&id);
    }

    pub fn get_passive(&self, id: ID) -> Option<&Passive> {
        self.passives.get(&id)
    }

    pub fn get_passive_mut(&mut self, id: ID) -> Option<&mut Passive> {
        self.passives.get_mut(&id)
    }

    pub fn remove_notebook(&mut self, id: ID) {
        self.notebooks.swap_remove(&id);
    }

    pub fn add_charge_pool(&mut self, charge_pool: ChargePool) -> ID {
        let id = self.next_charge_pool_id;
        self.next_charge_pool_id += 1;
        self.charge_pools.insert(id, charge_pool);
        id
    }

    pub fn remove_charge_pool(&mut self, id: ID) {
        self.charge_pools.swap_remove(&id);
    }

    pub fn get_charge_pool(&self, id: ID) -> Option<&ChargePool> {
        self.charge_pools.get(&id)
    }

    pub fn get_charge_pool_mut(&mut self, id: ID) -> Option<&mut ChargePool> {
        self.charge_pools.get_mut(&id)
    }

    pub fn get_poll(&self, id: ID) -> Option<&Poll> {
        self.polls.get(&id)
    }

    pub fn get_poll_mut(&mut self, id: ID) -> Option<&mut Poll> {
        self.polls.get_mut(&id)
    }

    pub fn add_poll(&mut self, poll: Poll) -> ID {
        let id = self.next_poll_id;
        self.next_poll_id += 1;
        self.polls.insert(id, poll);
        id
    }

    pub fn remove_poll(&mut self, id: ID) -> bool {
        self.polls.swap_remove(&id).is_some()
    }
}
