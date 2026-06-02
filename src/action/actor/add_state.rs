/*
* SYSTEM ACTION
* Add states and any associated restrictions found in engine config to an actor
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

#[derive(PartialEq, Eq, Clone)]
pub struct AddStateResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct AddState {
    pub actor_id: ID,
    pub state: State,
}

pub fn state_addition(actor_id: ID, state: State) -> Action {
    Action::AddState(AddState { actor_id, state })
}

impl ActionInterface for AddState {
    fn handle(
        &mut self,
        eng: &mut Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: Version,
        mutate: bool,
    ) -> ActionResult {
        actor.admin_or_system()?;

        let restrictions = eng
            .config
            .state_modifiers
            .get(&self.state)
            .cloned()
            .unwrap_or_default();

        let target = get_actor_mut(eng, self.actor_id)?;
        if mutate {
            target.add_state(self.state, restrictions);
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

        Ok(ActionResponse::AddState(AddStateResponse {}))
    }
}
