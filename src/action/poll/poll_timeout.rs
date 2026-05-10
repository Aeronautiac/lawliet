/*
* SYSTEM ACTION
* Handle a poll timeout
* (try to resolve the poll, if it accepts, execute, else, just delete it)
*/

use crate::{
    ID,
    action::{ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult},
    helpers::get_poll,
    poll::PolicyResult,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PollTimeoutResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PollTimeout {
    pub poll_id: ID,
}

impl ActionInterface for PollTimeout {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> ActionResult {
        actor.require_system()?;

        let poll = get_poll(eng, self.poll_id)?;
        let mut payload = poll.payload.clone();
        if payload.validate(eng, ctx, actor, version).is_err() {
            // TODO:
            // Tell frontend to acknowledge action failure (this should never happen in
            // practice. The poll will update and fail beforehand.)
        } else {
            let poll = get_poll(eng, self.poll_id).unwrap();
            let policy_res = poll.timeout_policy(eng);
            match policy_res {
                PolicyResult::Accept => {
                    payload.handle(eng, ctx, actor, version, mutate)?;
                    if mutate {
                        eng.world.remove_poll(self.poll_id);
                    }
                }
                PolicyResult::Reject => {
                    // TODO:
                    // tell frontend to acknowledge rejection
                }
                PolicyResult::Inconclusive => {
                    // TODO:
                    // tell frontend to acknowledge rejection
                }
            };
        }

        if mutate {
            eng.world.remove_poll(self.poll_id);
        }

        Ok(ActionResponse::PollTimeout(PollTimeoutResponse {}))
    }
}
