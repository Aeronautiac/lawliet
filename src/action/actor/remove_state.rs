/*
* SYSTEM ACTION
* Remove a state and its associated restrictions from an actor
*/

use crate::{
    ID,
    action::{
        Action, ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult,
        comms::update_contact_channels::UpdateContactChannels,
        world::update_world_channel_perms::UpdateWorldChannelPerms,
    },
    actor::state::State,
    common::Version,
    engine::Engine,
    helpers::{get_actor_mut, get_player},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct RemoveStateResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct RemoveState {
    pub actor_id: ID,
    pub state: State,
}

impl ActionInterface for RemoveState {
    fn handle(
        &mut self,
        eng: &mut Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: Version,
        mutate: bool,
    ) -> ActionResult {
        actor.admin_or_system()?;

        let target = get_actor_mut(eng, self.actor_id)?;
        if mutate {
            target.remove_state(self.state);
        }

        if get_player(eng, self.actor_id).is_ok() {
            Action::UpdateContactChannels(UpdateContactChannels {
                player_id: self.actor_id,
            })
            .handle(eng, ctx, actor, version, mutate)?;

            Action::UpdateWorldChannelPerms(UpdateWorldChannelPerms {
                player_id: self.actor_id,
            })
            .handle(eng, ctx, actor, version, mutate)?;
        }

        Ok(ActionResponse::RemoveState(RemoveStateResponse {}))
    }
}
