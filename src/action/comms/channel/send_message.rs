/*
* PLAYER ACTION
* Send a message to a channel
*/

use crate::{
    ID,
    action::{ActionError, ActionInterface, ActionResponse},
    channel::{ChannelPermission, SenderDisplay},
    helpers::{get_channel, player_id},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SendMessageResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SendMessage {
    pub channel_id: ID,
    pub display: SenderDisplay,
    pub content: String,
}

impl ActionInterface for SendMessage {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.player_only()?;
        let id = player_id(actor).expect("expected valid player id");

        let channel = get_channel(eng, id)?;
        let member = channel.get_member(id);
        let Some(member_data) = member else {
            return Err(ActionError::NotAChannelMember);
        };
        if !member_data.perms.contains(ChannelPermission::Send) {
            return Err(ActionError::InsufficientPermissions);
        }
        if !member_data.displays.contains(&self.display) {
            return Err(ActionError::DisplayNotOwned);
        }

        // repeating
        for player_id in channel.members.keys() {
            // TODO:
            // relay the message to everyone in the channel (including the sender) (frontend will likely handle the view context)
        }

        // relays
        // TODO:
        // loop through all bugs and relay the message to those as well if the channel is loggable and the bug
        // applies to the person who sent the message

        Ok(ActionResponse::SendMessage(SendMessageResponse {}))
    }
}
