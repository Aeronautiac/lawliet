/*
* SYSTEM ACTION
* Revive a dead player
*/

use crate::{
    ID,
    action::{
        Action, ActionActor, ActionInterface, ActionResponse, ActionResult, ResponseData,
        remove_state::RemoveState, require_dead,
    },
    actor::state::State,
    common::Version,
    engine::Engine,
};

#[derive(PartialEq, Eq, Clone)]
pub struct ReviveResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct Revive {
    pub target_id: ID,
}

impl ActionInterface for Revive {
    fn handle(
        &mut self,
        eng: &mut Engine,
        actor: &ActionActor,
        _: Version,
        _: bool,
    ) -> ActionResult {
        actor.require_system()?;
        require_dead(eng, self.target_id)?;

        Ok(ActionResponse {
            commands: vec![],
            next_actions: vec![Action::RemoveState(RemoveState {
                actor_id: self.target_id,
                state: State::Dead,
            })],
            data: ResponseData::Revive(ReviveResponse {}),
        })
    }
}
