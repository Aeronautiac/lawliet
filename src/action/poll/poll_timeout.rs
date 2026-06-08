/*
* SYSTEM ACTION
* Handle a poll timeout
* (try to resolve the poll, if it accepts, execute, else clean it up)
*/

use crate::{
    action::{
        Action, ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult,
        poll::poll_cleanup::PollCleanup,
    },
    common::{PollKey, Version},
    engine::Engine,
    helpers::get_poll,
    poll::PolicyResult,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PollTimeoutResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PollTimeout {
    pub poll_id: PollKey,
}

impl ActionInterface for PollTimeout {
    fn handle(
        &mut self,
        eng: &mut Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: Version,
        mutate: bool,
    ) -> ActionResult {
        actor.admin_or_system()?;

        let poll = get_poll(eng, self.poll_id)?;
        let mut payload = poll.payload.clone();
        let policy_res = poll.timeout_policy(eng);

        if payload.validate(eng, ctx, actor, version).is_err() {
            // TODO: tell frontend to acknowledge action failure
        } else {
            match policy_res {
                PolicyResult::Accept => {
                    payload.handle(eng, ctx, actor, version, mutate)?;
                }
                PolicyResult::Reject | PolicyResult::Inconclusive => {
                    // TODO: tell frontend to acknowledge rejection
                }
            }
        }

        Action::PollCleanup(PollCleanup {
            poll_id: self.poll_id,
            cancelled: false,
        })
        .handle(eng, ctx, actor, version, mutate)?;

        Ok(ActionResponse::PollTimeout(PollTimeoutResponse {}))
    }
}
