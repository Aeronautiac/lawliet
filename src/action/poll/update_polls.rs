/*
* SYSTEM ACTION
* Check all polls to see if they can be resolved. If they can, resolve them.
*/

use crate::{
    ID,
    action::{ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult},
    helpers::get_poll,
    poll::PolicyResult,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UpdatePollsResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UpdatePolls {}

impl ActionInterface for UpdatePolls {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> ActionResult {
        actor.require_system()?;

        let mut polls_to_cancel = vec![];
        let mut polls_to_accept = vec![];
        let mut polls_to_reject = vec![];
        let ids: Vec<ID> = eng.world.polls.keys().cloned().collect();
        for id in ids {
            let poll = get_poll(eng, id).unwrap();
            let mut payload = poll.payload.clone();
            if payload.validate(eng, ctx, actor, version).is_err() {
                polls_to_cancel.push(id);
            } else {
                let poll = get_poll(eng, id).unwrap();
                let policy_res = poll.update_policy(eng);
                match policy_res {
                    PolicyResult::Accept => polls_to_accept.push((id, payload)),
                    PolicyResult::Reject => polls_to_reject.push(id),
                    _ => {}
                };
            }
        }

        for id in polls_to_cancel {
            if mutate {
                eng.world.remove_poll(id);
            }
            // TODO:
            // - send command to frontend to acknowledge poll cancellation
        }

        for id in polls_to_reject {
            if mutate {
                eng.world.remove_poll(id);
            }
            // TODO:
            // - tell frontend to acknowledge poll rejection
        }

        // the actions are guaranteed to succeed by this point. if they dont, something's wrong.
        for (id, mut action) in polls_to_accept {
            action.handle(eng, ctx, actor, version, mutate)?;
            if mutate {
                eng.world.remove_poll(id);
            }
        }

        Ok(ActionResponse::UpdatePolls(UpdatePollsResponse {}))
    }
}
