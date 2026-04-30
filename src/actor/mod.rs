pub mod organization;
pub mod player;
pub mod restriction;
pub mod state;

use std::collections::{BTreeMap, BTreeSet};

pub use self::organization::Organization;
pub use self::player::Player;

use crate::{
    ID,
    actor::{
        restriction::Restrictions,
        state::{State, States},
    },
    config::role::Role,
};
use restriction::{Restriction, Source};

#[derive(PartialEq, Eq, Debug, Ord, PartialOrd, Clone, Copy)]
pub enum ActorLinkType {
    Life, // if an actor has a life link to another actor, then when the main actor dies, so will
    // the other actor, and the same is true for revivals.
    Passive, // if an actor has a passive link to another actor, the main actor is treated as if it
             // has the passives of the other actor
             // it is in this order because the reverse would require a full list traversal
             // passive links are severed on death
             // link death and revive behaviours can be explicitly ignored in their corresponding actions
}

#[derive(PartialEq, Eq, Debug, Ord, PartialOrd, Clone)]
pub struct ActorLink {
    pub link_type: ActorLinkType,
    pub link_dest: ID,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ActorType {
    System,
    Org(Organization),
    Player(Player),
}

#[derive(PartialEq, Eq, Debug)]
pub struct Actor {
    pub kills: Vec<ID>,
    pub abilities: BTreeSet<ID>, // true ability ownership is represented by the OwnershipStruct for both
    // passives and abilities. this only exists for performance. it must be correctly maintained.
    pub passives: BTreeSet<ID>,
    pub restrictions: BTreeMap<Source, Restrictions>,
    pub states: States,
    pub actor_type: ActorType,
    pub actor_links: BTreeSet<ActorLink>,
}

impl Actor {
    pub fn new_player(true_name: &str, role: Role) -> Self {
        Actor {
            kills: vec![],
            abilities: BTreeSet::new(),
            passives: BTreeSet::new(),
            restrictions: BTreeMap::new(),
            states: States::empty(),
            actor_links: BTreeSet::new(),
            actor_type: ActorType::Player(Player::new(true_name, role)),
        }
    }

    pub fn new_org() -> Self {
        Actor {
            kills: vec![],
            abilities: BTreeSet::new(),
            passives: BTreeSet::new(),
            restrictions: BTreeMap::new(),
            actor_links: BTreeSet::new(),
            states: States::empty(),
            actor_type: ActorType::Org(Organization::new()),
        }
    }

    pub fn add_restrictions(&mut self, source: Source, restrictions: Restrictions) {
        self.restrictions.insert(source, restrictions);
    }

    pub fn remove_restrictions(&mut self, source: Source) {
        self.restrictions.remove(&source);
    }

    pub fn has_restriction(&self, restriction: Restriction) -> bool {
        let mut restrictions = Restrictions::empty();
        for restrict in self.restrictions.values() {
            restrictions |= *restrict;
        }
        restrictions.contains(restriction)
    }

    // adds a state
    // if any restrictions are associated with the state, it also adds the restrictions
    pub fn add_state(&mut self, new_state: State, restrictions: Restrictions) {
        self.states.set(new_state, true);
        self.add_restrictions(Source::State(new_state), restrictions);
    }

    // removes a state
    // if any restrictions are associated with the state, it removes the restrictions
    pub fn remove_state(&mut self, remove_state: State) {
        self.states.set(remove_state, false);
        self.remove_restrictions(Source::State(remove_state));
    }

    pub fn add_link(&mut self, link: ActorLink) {
        self.actor_links.insert(link);
    }

    pub fn sever_link(&mut self, link: ActorLink) {
        self.actor_links.remove(&link);
    }

    pub fn remove_ability(&mut self, id: ID) {
        self.abilities.remove(&id);
    }

    pub fn add_ability(&mut self, id: ID) {
        self.abilities.insert(id);
    }

    pub fn remove_passive(&mut self, id: ID) {
        self.passives.remove(&id);
    }

    pub fn add_passive(&mut self, id: ID) {
        self.passives.insert(id);
    }
}
