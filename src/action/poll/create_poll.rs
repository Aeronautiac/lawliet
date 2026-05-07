/*
* SYSTEM ACTION
* Create a new poll
* (Box is fine because something like this should be as generic as possible for
* developer convenience. This action is rarely used anyway so pointer chasing isn't really a problem.)
*/

use crate::{
    ID,
    action::{Action, ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult},
    poll::{Poll, PollPolicy, PollVisibility, VoterPolicy},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CreatePollReponse {
    pub id: ID,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CreatePoll {
    pub voter_policy: VoterPolicy,
    pub visibility: PollVisibility,
    pub update_policy: PollPolicy,
    pub timeout_policy: PollPolicy,
    pub payload: Box<Action>,
}

impl ActionInterface for CreatePoll {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> ActionResult {
        actor.require_system()?;

        let id = if mutate {
            eng.world.add_poll(Poll::new(
                *(self.payload.clone()),
                self.visibility,
                self.update_policy,
                self.timeout_policy,
                self.voter_policy,
            ))
        } else {
            0
        };

        Ok(ActionResponse::CreatePoll(CreatePollReponse { id }))
    }
}
