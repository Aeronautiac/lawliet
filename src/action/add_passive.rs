/*
* SYSTEM ACTION
* Add a passive to the world
*/

use crate::{
    ID,
    ability::Ability,
    action::{ActionContext, ActionError, ActionInterface, ActionResponse},
    common::Variant,
    config::ability::{AbilityIdentifier, AbilityName},
    ownership::OwnershipStruct,
    passive::{Passive, PassiveType},
};

#[derive(PartialEq, Eq, Clone)]
pub struct AddPassiveResponse {
    pub id: ID,
}

#[derive(PartialEq, Eq, Clone)]
pub struct AddPassive {
    pub passive_type: PassiveType,
    pub transferrable: bool,
}

impl ActionInterface for AddPassive {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut ActionContext,
        actor: &super::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> super::ActionResult {
        actor.require_system()?;

        let id = if mutate {
            let passive = Passive {
                ownership_struct: OwnershipStruct {
                    owner: None,
                    transferrable: self.transferrable,
                    volatile: false,
                },
                passive_type: self.passive_type,
            };
            eng.world.add_passive(passive)
        } else {
            0
        };

        Ok(ActionResponse::AddPassive(AddPassiveResponse { id }))
    }
}
