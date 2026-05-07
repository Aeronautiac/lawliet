/*
* SYSTEM ACTION
* Add a passive to the world
*/

use crate::{
    ID,
    action::{ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult},
    ownership::OwnershipStruct,
    passive::{Passive, PassiveType},
};

#[derive(PartialEq, Eq, Clone)]
pub struct AddPassiveResponse {
    pub id: ID,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct AddPassive {
    pub passive_type: PassiveType,
    pub transferrable: bool,
}

impl ActionInterface for AddPassive {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> ActionResult {
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
