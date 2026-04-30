/*
* SYSTEM ACTION
* Transfer ownership of an ability to a specified actor and then reset links
*/

use crate::{
    ID,
    action::{ActionContext, ActionInterface, ActionResponse, get_actor, get_passive_mut},
};

#[derive(PartialEq, Eq, Clone)]
pub struct GivePassiveResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct GivePassive {
    pub passive_id: ID,
    pub actor_id: ID,
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

        let passive = get_passive_mut(eng, self.passive_id)?;
        if mutate {
            passive.ownership_struct.set_owner(self.actor_id);
        }

        Ok(ActionResponse::GivePassive(GivePassiveResponse {}))
    }
}
