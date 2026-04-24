// Two layers of indirection:
// - UseAbility action (checks the generalized ability data along with ability specific validation)
// - specific abilities are implemented as structs with ability specific arguments and a validate + execute function. execute
// returns a vector of actions and validate returns an action error or ok.
// - dispatch is done using enum_dispatch. ability handlers are analogous to action handlers with
// slight differences in specifics.
// - action handler structs return the ability name enum through a method
//
// Ex:
// 1. UseAbility called with a specialized struct in an ability usage enum/
// 2. UseAbility calls the struct's functions and returns its results

use crate::{
    action::{Action, ActionActor, ActionError},
    config::ability::AbilityName,
    engine::Engine,
};

pub trait AbilityInterface {
    fn ability_name() -> AbilityName;
    fn validate(
        &self,
        eng: &Engine,
        actor: &ActionActor,
        ability: &Ability,
    ) -> Result<(), ActionError>;
    fn execute(self, eng: &mut Engine, actor: &ActionActor, ability: Ability) -> Vec<Action>;
}

#[derive(Debug)]
pub struct Ability {
    pub charges: u8,
    pub iterations_to_reset: u8, // the number of iterations until charges are reset
    pub ability_name: AbilityName, // the other stuff about the ability (like its category) is
    // determined by the config struct
    pub variant: u8, // 0 by default. use associated constants to define meanings in different abilities.
}

impl Ability {}
