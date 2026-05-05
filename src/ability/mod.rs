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

use crate::{
    ID,
    ability::{
        gun::{Gun, GunResponse},
        pseudocide::{Pseudocide, PseudocideResponse},
    },
    action::{ActionActor, ActionContext, ActionError},
    chargepool::{PoolLink, PoolLinkType},
    common::{LinkWeight, Variant},
    config::ability::AbilityName,
    engine::Engine,
    ownership::OwnershipStruct,
};
use enum_dispatch::enum_dispatch;
use indexmap::IndexSet;

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

// a volatile link is destroyed when the owner changes
#[derive(Hash, PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub struct AbilityPoolLink {
    pub volatile: bool,
    pub link: PoolLink,
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

type AbilityResult = Result<AbilityResponse, ActionError>;

// if an ability has no pool links, then it has infinite usages
#[derive(Debug)]
pub struct Ability {
    pub ownership_struct: OwnershipStruct,
    pub pool_links: IndexSet<AbilityPoolLink>, // charge pools
    pub ability_name: AbilityName, // the other stuff about the ability (like its category) is
    // determined by the config struct
    pub variant: Variant, // 0 by default. use associated constants to define meanings in different abilities.
                          // variants also have meanings in config files.
}

impl Ability {
    pub fn new(ability_name: AbilityName, variant: Variant, transferrable: bool) -> Self {
        Ability {
            pool_links: IndexSet::new(),
            variant,
            ability_name,
            ownership_struct: OwnershipStruct::new(transferrable),
        }
    }

    pub fn add_link(
        &mut self,
        link_dest: ID,
        link_type: PoolLinkType,
        weight: LinkWeight,
        volatile: bool,
    ) -> Option<AbilityPoolLink> {
        let removed: Option<AbilityPoolLink> = self
            .pool_links
            .extract_if(.., |l| l.link.link_dest == link_dest)
            .next();
        self.pool_links.insert(AbilityPoolLink {
            volatile,
            link: PoolLink {
                link_type,
                link_dest,
                weight,
            },
        });
        removed
    }

    pub fn remove_link(&mut self, link_dest: ID) {
        self.pool_links.retain(|l| l.link.link_dest != link_dest)
    }
}
