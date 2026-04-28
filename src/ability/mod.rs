// Two layers of indirection:
// - UseAbility action (checks the generalized ability data along with ability specific validation)
// - specific abilities are implemented as structs with ability specific arguments and a handle function. execute
// - dispatch is done using enum_dispatch. ability handlers are analogous to action handlers with
// slight differences in specifics.
// - action handler structs return the ability name enum through a method
//
// Ex:
// 1. UseAbility called with a specialized struct in an ability usage enum/
// 2. UseAbility calls the struct's functions and returns its results

use std::{
    collections::{BTreeMap, BTreeSet},
    process::Command,
};

use crate::{
    ID,
    ability::pseudocide::{Pseudocide, PseudocideResponse},
    action::{Action, ActionActor, ActionError},
    common::{ChargeCount, IterationCount, LinkWeight, Variant},
    config::ability::AbilityName,
    engine::Engine,
};
use enum_dispatch::enum_dispatch;

pub mod pseudocide;

#[enum_dispatch]
pub trait AbilityInterface {
    fn ability_name(&self) -> AbilityName;
    fn handle(
        &mut self,
        eng: &mut Engine,
        actor: &ActionActor,
        ability: &mut Ability,
        version: u8,
        mutate: bool,
    ) -> AbilityResult;
}

#[enum_dispatch(AbilityInterface)]
pub enum AbilityBehaviour {
    Pseudocide(Pseudocide),
}

pub enum AbilityResponseData {
    Pseudocide(PseudocideResponse),
}

pub struct AbilityResponse {
    commands: Vec<Command>,
    actions: Vec<Action>,
    data: AbilityResponseData,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy)]
pub enum AbilityLinkType {
    Limit, // the ability is limited by the ability linked to (if at least one limit ability has 0 charges,
    // then the ability cannot be used)
    Pool, // the ability takes charges out of a pool of linked abilities (if at least one pool
          // ability has > 0 charges remaining, the ability can be used)
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub struct AbilityLink {
    pub link_type: AbilityLinkType,
    pub link_dest: ID,
    pub weight: LinkWeight,
}

type AbilityResult = Result<AbilityResponse, ActionError>;

#[derive(Debug)]
pub struct Ability {
    pub owner: Option<ID>, // the actor which this ability is owned by (if any)
    pub links: BTreeSet<AbilityLink>, // any ability in this set gates this ability (this ability's
    // charges dont matter if the other ability has none). this ability will also decrement the
    // charges of the linked ability on success. links may be unidirectional (the presence of a link in
    // one ability does not imply the presence of another link in the ability linked to although possible).
    pub charges: ChargeCount, // how many times the ability can be used
    pub iterations_to_reset: IterationCount, // the number of iterations until charges are reset
    pub ability_name: AbilityName, // the other stuff about the ability (like its category) is
    // determined by the config struct
    pub variant: Variant, // 0 by default. use associated constants to define meanings in different abilities.
    // variants also have meanings in config files.
    pub volatile: bool, // determines whether or not the ability is deleted when the owner changes
                        // significantly (i.e., the role changes)
}

impl Ability {
    pub fn new(
        ability_name: AbilityName,
        variant: Variant,
        charges: ChargeCount,
        volatile: bool,
    ) -> Self {
        Ability {
            owner: None,
            links: BTreeSet::new(),
            iterations_to_reset: 0,
            charges,
            variant,
            volatile,
            ability_name,
        }
    }

    /// checks if some person can use the ability
    pub fn can_be_used(&self, user: ID) -> bool {
        if let Some(owner_id) = self.owner
            && owner_id == user
        {
            self.charges > 0
        } else {
            false
        }
    }

    /// creates a new ability link
    /// if there is already a link to an ability, it replaces it
    /// return the old link to the ability (if any)
    pub fn add_link(
        &mut self,
        link_dest: ID,
        link_type: AbilityLinkType,
        weight: LinkWeight,
    ) -> Option<AbilityLink> {
        let removed: Option<AbilityLink> = self
            .links
            .extract_if(.., |l| l.link_dest == link_dest)
            .next();
        self.links.insert(AbilityLink {
            link_type,
            link_dest,
            weight,
        });
        removed
    }

    pub fn remove_link(&mut self, link_dest: ID) {
        self.links.retain(|l| l.link_dest != link_dest)
    }

    pub fn set_owner(&mut self, id: ID) {
        self.owner = Some(id);
    }

    pub fn clear_links(&mut self) {
        self.links = BTreeSet::new();
    }
}
