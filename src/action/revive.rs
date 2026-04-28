/*
* SYSTEM ACTION
* Revive a dead player
*/

use crate::{
    ID,
    action::{
        Action, ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult,
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
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: Version,
        mutate: bool,
    ) -> ActionResult {
        actor.require_system()?;
        require_dead(eng, self.target_id)?;

        Action::RemoveState(RemoveState {
            actor_id: self.target_id,
            state: State::Dead,
        })
        .handle(eng, ctx, actor, version, mutate)?;

        Ok(ActionResponse::Revive(ReviveResponse {}))
    }
}
