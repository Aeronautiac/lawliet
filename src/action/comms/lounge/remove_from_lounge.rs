/*
* SYSTEM ACTION
* Remove a player from a lounge
*/

use crate::{
    ID,
    action::{
        Action, ActionError, ActionInterface, ActionResponse, comms::channel::set_member::SetMember,
    },
    helpers::{get_channel, get_channel_mut, get_lounge, get_player, get_player_mut},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct RemoveFromLoungeResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct RemoveFromLounge {
    pub lounge_id: ID,
    pub player_id: ID,
}

impl ActionInterface for RemoveFromLounge {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.require_system()?;

        let player = get_player(eng, self.player_id)?;
        if !player.lounges.contains(&self.lounge_id) {
            return Err(ActionError::PlayerNotInLounge);
        }

        let lounge = get_lounge(eng, self.lounge_id)?;
        Action::SetMember(SetMember {
            player_id: self.player_id,
            channel_id: lounge.channel_id,
            settings: None,
        })
        .handle(eng, ctx, actor, version, mutate)?;

        if mutate {
            let player = get_player_mut(eng, self.player_id)?;
            player.remove_lounge(self.lounge_id);
        }

        Ok(ActionResponse::RemoveFromLounge(
            RemoveFromLoungeResponse {},
        ))
    }
}
