pub mod organization;
pub mod player;
pub mod restriction;
pub mod state;

use std::collections::BTreeMap;

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

#[derive(PartialEq, Eq, Debug)]
pub enum ActorType {
    System,
    Org(Organization),
    Player(Player),
}

#[derive(PartialEq, Eq, Debug)]
pub struct Actor {
    pub kills: Vec<ID>,
    pub restrictions: BTreeMap<Source, Restrictions>,
    pub states: States,
    pub actor_type: ActorType,
}

impl Actor {
    pub fn new_player(true_name: &str, role: Role) -> Self {
        Actor {
            kills: vec![],
            restrictions: BTreeMap::new(),
            states: States::empty(),
            actor_type: ActorType::Player(Player::new(true_name, role)),
        }
    }

    pub fn new_org() -> Self {
        Actor {
            kills: vec![],
            restrictions: BTreeMap::new(),
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
}
