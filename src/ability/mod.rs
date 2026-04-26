// Two layers of indirection:
// - UseAbility action (checks the generalized ability data along with ability specific validation)
// - specific abilities are implemented as structs with ability specific arguments and a handle function. execute
// returns a vector of actions and validate returns an action error or ok.
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
    ability::pseudocide::{Pseudocide, PseudocideResponse},
    action::{Action, ActionActor, ActionError},
    config::ability::AbilityName,
    engine::Engine,
};
use enum_dispatch::enum_dispatch;

pub mod pseudocide;

// need more flexibility. likely need some kind of ability response data as well. it will be wrapped
// by use_ability response data. there is also the issue of abilities potentially being linked to one
// another in certain cases (such as anonymous contact being an alternate path to contact). how would this be represented?
// by the structure of the engine, it is likely possible to just return another use_ability from an ability
// handler. dry run requires everything to pass the validation phase (including the next ability usage).
// another problem: if anonymous contact relies on contact, then how does contact alter its
// behaviour to reflect this? the answer is likely that contact is not an ability in of itself. it
// is an action. indivual abilities have a "link" set which is just a set of ability ids which it
// depends on the charges of. it will also decrement charges on usage. this is handled within the
// use_ability action.
// then, anonymous contact and contact are simply wrappers for the underlying contact action. their
// resources are linked through the link set.
// the only problem with this is that the config struct must be able to model default relationships
// between abilities.
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
    Psuedocide(PseudocideResponse),
}

pub struct AbilityResponse {
    actions: Vec<Action>,
    data: AbilityResponseData,
}

type AbilityResult = Result<AbilityResponse, ActionError>;

#[derive(Debug)]
pub struct Ability {
    pub owner: Option<ID>, // the actor which this ability is owned by (if any)
    pub resource_links: BTreeSet<ID>, // any ability in this set gates this ability (this ability's
    // charges dont matter if the other ability has none). this ability will also decrement the
    // charges of the linked ability on success. links may be unidirectional (the presence of a link in
    // one ability does not imply the presence of another link in the ability linked to although possible).
    pub charges: u8,               // how many times the ability can be used
    pub iterations_to_reset: u8,   // the number of iterations until charges are reset
    pub ability_name: AbilityName, // the other stuff about the ability (like its category) is
    // determined by the config struct
    pub variant: u8, // 0 by default. use associated constants to define meanings in different abilities.
    pub volatile: bool, // determines whether or not the ability is deleted when the owner changes
                     // significantly (i.e., the role changes)
}

impl Ability {}
