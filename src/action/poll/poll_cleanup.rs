/*
* SYSTEM ACTION
* Remove a poll from the world after it has concluded or been cancelled.
* Called by PollTimeout (natural conclusion) and TerminateProsecution (cancellation).
*
* TODO: commands (use `cancelled` to differentiate frontend feedback)
*/

use crate::{
    action::{ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult},
    common::{PollKey, Version},
    engine::Engine,
    helpers::get_poll,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PollCleanupResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PollCleanup {
    pub poll_id: PollKey,
    pub cancelled: bool,
}

impl ActionInterface for PollCleanup {
    fn handle(
        &mut self,
        eng: &mut Engine,
        _ctx: &mut ActionContext,
        actor: &ActionActor,
        _version: Version,
        mutate: bool,
    ) -> ActionResult {
        actor.admin_or_system()?;
        get_poll(eng, self.poll_id)?;

        if mutate {
            eng.world.remove_poll(self.poll_id);
        }

        Ok(ActionResponse::PollCleanup(PollCleanupResponse {}))
    }
}
