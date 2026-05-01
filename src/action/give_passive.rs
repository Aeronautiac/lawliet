/*
* SYSTEM ACTION
* Transfer ownership of an ability to a specified actor and then reset links
*/

use crate::{
    ID,
    action::{ActionContext, ActionError, ActionInterface, ActionResponse},
    helpers::{get_actor, get_actor_mut, get_passive, get_passive_mut},
};

#[derive(PartialEq, Eq, Clone)]
pub struct GivePassiveResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct GivePassive {
    pub passive_id: ID,
    pub actor_id: ID,
    pub volatile: bool,
}

impl ActionInterface for GivePassive {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut ActionContext,
        actor: &super::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> super::ActionResult {
        actor.require_system()?;
        get_actor(eng, self.actor_id)?;

        let passive = get_passive(eng, self.passive_id)?;
        if let Some(owner) = passive.ownership_struct.owner {
            if owner == self.actor_id {
                return Err(ActionError::ItemAlreadyOwned);
            }
            if mutate {
                let other_actor = get_actor_mut(eng, owner).unwrap(); // should
                // crash if the owner is an actor that doesnt exist (the state is invalid)
                other_actor.remove_passive(self.passive_id);
            }
        }

        let passive = get_passive_mut(eng, self.passive_id)?;
        if mutate {
            passive
                .ownership_struct
                .set_owner(self.actor_id, self.volatile);
            let actor_data = get_actor_mut(eng, self.actor_id)?;
            actor_data.add_passive(self.passive_id);
        }

        Ok(ActionResponse::GivePassive(GivePassiveResponse {}))
    }
}
