/*
* SYSTEM ACTION
* Remove a state and its associated restrictions from an actor
*/

use crate::{
    ID,
    action::{ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult},
    actor::state::State,
    common::Version,
    engine::Engine,
    helpers::get_actor_mut,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct RemoveStateResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct RemoveState {
    pub actor_id: ID,
    pub state: State,
}

impl ActionInterface for RemoveState {
    fn handle(
        &mut self,
        eng: &mut Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        _: Version,
        mutate: bool,
    ) -> ActionResult {
        actor.require_system()?;

        let target = get_actor_mut(eng, self.actor_id)?;
        if mutate {
            target.remove_state(self.state);
        }

        Ok(ActionResponse::RemoveState(RemoveStateResponse {}))
    }
}
