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
    actor::{Actor, ActorType, Player, organization::LeadershipStruct},
    bug::Bug,
    channel::Channel,
    chargepool::ChargePool,
    config::{actor::organization::OrganizationName, role::Role, world::WorldChargePoolName},
    groupchat::Groupchat,
    lounge::Lounge,
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
    pub channels: IndexMap<ID, Channel>,
    pub lounges: IndexMap<ID, Lounge>,
    pub groupchats: IndexMap<ID, Groupchat>,
    pub bugs: IndexMap<ID, Bug>,
    next_charge_pool_id: ID,
    next_actor_id: ID,
    next_notebook_id: ID,
    next_ability_id: ID,
    next_passive_id: ID,
    next_poll_id: ID,
    next_channel_id: ID,
    next_lounge_id: ID,
    next_groupchat_id: ID,
    next_bug_id: ID,
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
            channels: IndexMap::new(),
            lounges: IndexMap::new(),
            groupchats: IndexMap::new(),
            bugs: IndexMap::new(),
            next_charge_pool_id: 0,
            next_actor_id: 0,
            next_notebook_id: 0,
            next_ability_id: 0,
            next_passive_id: 0,
            next_poll_id: 0,
            next_channel_id: 0,
            next_lounge_id: 0,
            next_groupchat_id: 0,
            next_bug_id: 0,
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

    pub fn add_org(
        &mut self,
        name: OrganizationName,
        leadership_struct: Option<LeadershipStruct>,
    ) -> ID {
        let id = self.next_actor_id;
        self.next_actor_id += 1;
        self.actors
            .insert(id, Actor::new_org(name, leadership_struct));
        id
    }

    pub fn add_notebook(&mut self, channel_id: ID, fake: bool) -> ID {
        let id = self.next_notebook_id;
        self.next_notebook_id += 1;
        self.notebooks.insert(id, Notebook::new(channel_id, fake));
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

    pub fn add_channel(&mut self, channel: Channel) -> ID {
        let id = self.next_channel_id;
        self.next_channel_id += 1;
        self.channels.insert(id, channel);
        id
    }

    pub fn remove_channel(&mut self, id: ID) -> bool {
        self.channels.swap_remove(&id).is_some()
    }

    pub fn get_channel(&self, id: ID) -> Option<&Channel> {
        self.channels.get(&id)
    }

    pub fn get_channel_mut(&mut self, id: ID) -> Option<&mut Channel> {
        self.channels.get_mut(&id)
    }

    pub fn add_lounge(&mut self, lounge: Lounge) -> ID {
        let id = self.next_lounge_id;
        self.next_lounge_id += 1;
        self.lounges.insert(id, lounge);
        id
    }

    pub fn get_lounge(&self, id: ID) -> Option<&Lounge> {
        self.lounges.get(&id)
    }

    pub fn get_lounge_mut(&mut self, id: ID) -> Option<&mut Lounge> {
        self.lounges.get_mut(&id)
    }

    pub fn add_groupchat(&mut self, gc: Groupchat) -> ID {
        let id = self.next_groupchat_id;
        self.next_groupchat_id += 1;
        self.groupchats.insert(id, gc);
        id
    }

    pub fn get_groupchat(&self, id: ID) -> Option<&Groupchat> {
        self.groupchats.get(&id)
    }

    pub fn get_groupchat_mut(&mut self, id: ID) -> Option<&mut Groupchat> {
        self.groupchats.get_mut(&id)
    }

    pub fn add_bug(&mut self, bug: Bug) -> ID {
        let id = self.next_bug_id;
        self.next_bug_id += 1;
        self.bugs.insert(id, bug);
        id
    }

    pub fn get_bug(&self, id: ID) -> Option<&Bug> {
        self.bugs.get(&id)
    }

    pub fn get_bug_mut(&mut self, id: ID) -> Option<&mut Bug> {
        self.bugs.get_mut(&id)
    }
}
