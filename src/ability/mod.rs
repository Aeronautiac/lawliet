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

use std::collections::BTreeSet;

use crate::{
    ID,
    ability::{
        gun::{Gun, GunResponse},
        pseudocide::{Pseudocide, PseudocideResponse},
    },
    action::{ActionActor, ActionContext, ActionError},
    common::{ChargeCount, IterationCount, LinkWeight, Variant},
    config::ability::AbilityName,
    engine::Engine,
    ownership::OwnershipStruct,
};
use enum_dispatch::enum_dispatch;

pub mod gun;
pub mod pseudocide;

#[enum_dispatch]
pub trait AbilityInterface {
    fn ability_name(&self) -> AbilityName;
    fn handle(
        &mut self,
        eng: &mut Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        ability: ID,
        version: u8,
        mutate: bool,
    ) -> AbilityResult;
}

#[enum_dispatch(AbilityInterface)]
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub enum AbilityBehaviour {
    Pseudocide(Pseudocide),
    Gun(Gun),
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub enum AbilityResponse {
    Pseudocide(PseudocideResponse),
    Gun(GunResponse),
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy)]
pub enum AbilityLinkType {
    Limit, // every linked ability loses charges with the amount depending on weight. if at least
    // one ability cannot afford it, then usage fails.
    Pool, // same subtraction policy, but only fails of none of the linked abilities can afford the
          // cost.
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
    pub ownership_struct: OwnershipStruct,
    pub links: BTreeSet<AbilityLink>, // any ability in this set gates this ability (this ability's
    // charges dont matter if the other ability has none). this ability will also decrement the
    // charges of the linked ability on success, and the magnitude of the decrement is based on the link weight.
    // links may be unidirectional (the presence of a link in one ability does not imply the presence config
    // another link in the ability linked to although possible).
    pub charges: ChargeCount, // how many times the ability can be used
    pub iterations_to_reset: IterationCount, // the number of iterations until charges are reset
    pub ability_name: AbilityName, // the other stuff about the ability (like its category) is
    // determined by the config struct
    pub variant: Variant, // 0 by default. use associated constants to define meanings in different abilities.
                          // variants also have meanings in config files.
}

impl Ability {
    pub fn new(
        ability_name: AbilityName,
        variant: Variant,
        charges: ChargeCount,
        transferrable: bool,
    ) -> Self {
        Ability {
            links: BTreeSet::new(),
            iterations_to_reset: 0,
            charges,
            variant,
            ability_name,
            ownership_struct: OwnershipStruct::new(transferrable),
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

    pub fn has_charges(&self) -> bool {
        self.charges > 0
    }

    pub fn on_use(&mut self, reset_time: IterationCount) {
        self.charges -= 1;
        if self.iterations_to_reset == 0 {
            self.iterations_to_reset = reset_time
        }
    }

    pub fn remove_link(&mut self, link_dest: ID) {
        self.links.retain(|l| l.link_dest != link_dest)
    }

    pub fn clear_links(&mut self) {
        self.links = BTreeSet::new();
    }
}
