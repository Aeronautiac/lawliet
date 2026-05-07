/*
* PLAYER ONLY
* Add a vote to a poll
*/

use crate::{
    ID,
    action::{ActionError, ActionInterface, ActionResponse},
    helpers::{actor_id, get_poll, get_poll_mut},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct AddVoteResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct AddVote {
    pub poll_id: ID,
    pub accept: bool,
}

impl ActionInterface for AddVote {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.player_only()?;
        let player_id = actor_id(actor).unwrap();

        let poll = get_poll(eng, self.poll_id)?;
        if !poll.voter_policy(eng, player_id) {
            return Err(ActionError::InvalidVoter);
        }
        if mutate {
            let poll = get_poll_mut(eng, self.poll_id)?;
            poll.add_vote(player_id, self.accept);
        }

        Ok(ActionResponse::AddVote(AddVoteResponse {}))
    }
}
