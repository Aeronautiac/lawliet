/*
* SYSTEM ACTION
* Remove a state and its associated restrictions from an actor
*/

use crate::{
    ID,
    action::{
        ActionActor, ActionInterface, ActionResponse, ActionResult, ResponseData, get_actor_mut,
    },
    actor::state::State,
    common::Version,
    engine::Engine,
};

#[derive(PartialEq, Eq, Clone)]
pub struct RemoveStateResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct RemoveState {
    pub actor_id: ID,
    pub state: State,
}

impl ActionInterface for RemoveState {
    fn handle(
        &mut self,
        eng: &mut Engine,
        actor: &ActionActor,
        _: Version,
        mutate: bool,
    ) -> ActionResult {
        actor.require_system()?;

        let target = get_actor_mut(eng, self.actor_id)?;
        if mutate {
            target.remove_state(self.state);
        }

        Ok(ActionResponse {
            commands: vec![],
            next_actions: vec![],
            data: ResponseData::RemoveState(RemoveStateResponse {}),
        })
    }
}
