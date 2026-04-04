pub mod organization;
pub mod player;
pub mod restrictions;
pub mod states;

use std::collections::{BTreeMap, BTreeSet};

pub use self::organization::Organization;
pub use self::player::Player;

use crate::{ID, actor::states::States};
use restrictions::{Restrictions, Source};

#[derive(PartialEq, Eq)]
pub enum ActorType {
    System,
    Org(Organization),
    Player(Player),
}

#[derive(PartialEq, Eq)]
pub struct Actor {
    abilities: Vec<ID>,
    kills: Vec<ID>,
    restrictions: BTreeMap<Source, Restrictions>,
    states: States,
    actor_type: ActorType,
}
