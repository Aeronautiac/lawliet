/*
* SYSTEM ACTION
* Revive a dead player
*/

use crate::{
    ID,
    action::{
        Action, ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult,
        get_actor, remove_state::RemoveState, require_dead,
    },
    actor::{ActorLinkType, state::State},
    common::Version,
    engine::Engine,
};

#[derive(PartialEq, Eq, Clone)]
pub struct ReviveResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct Revive {
    pub ignore_links: bool,
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

        let mut next_actions = vec![];
        if !self.ignore_links {
            let actor = get_actor(eng, self.target_id)?;
            let links = actor.actor_links.clone();
            for link in links {
                if link.link_type == ActorLinkType::Life {
                    let other_actor = get_actor(eng, link.link_dest)?;
                    if other_actor.states.contains(State::Dead) {
                        next_actions.push(Action::Revive(Revive {
                            ignore_links: false,
                            target_id: link.link_dest,
                        }));
                    }
                }
            }
        }
        for mut action in next_actions {
            action.handle(eng, ctx, actor, version, mutate)?;
        }

        Ok(ActionResponse::Revive(ReviveResponse {}))
    }
}
