/*
* PLAYER ONLY
* Remove a vote from a poll
*/

use crate::{
    ID,
    action::{ActionError, ActionInterface, ActionResponse},
    helpers::{actor_id, get_poll, get_poll_mut},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct RemoveVoteResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct RemoveVote {
    pub poll_id: ID,
}

impl ActionInterface for RemoveVote {
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
        if !poll.contains_voter(player_id) {
            return Err(ActionError::NotAVoter);
        }
        if mutate {
            let poll = get_poll_mut(eng, self.poll_id)?;
            poll.remove_vote(player_id);
        }

        Ok(ActionResponse::RemoveVote(RemoveVoteResponse {}))
    }
}
