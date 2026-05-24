/*
* SYSTEM ACTION
* Map a player ID to a channel member struct within the channel
*/

use crate::{
    ID,
    action::{ActionInterface, ActionResponse},
    channel::ChannelMember,
    helpers::{get_channel_mut, get_player},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SetMemberResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SetMember {
    pub player_id: ID,
    pub channel_id: ID,
    pub settings: Option<ChannelMember>,
}

// TODO:
// handle sending out the right commands

impl ActionInterface for SetMember {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.require_system()?;
        get_player(eng, self.player_id)?;

        let channel = get_channel_mut(eng, self.channel_id)?;
        if mutate {
            channel.set_member(self.player_id, self.settings.clone());
        }

        Ok(ActionResponse::SetMember(SetMemberResponse {}))
    }
}
