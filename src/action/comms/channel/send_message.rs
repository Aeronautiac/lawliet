/*
* PLAYER ACTION
* Send a message to a channel
*/

use crate::{
    ID,
    action::{ActionError, ActionInterface, ActionResponse},
    channel::ChannelPermission,
    helpers::{get_channel, player_id},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SendMessageResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SendMessage {
    pub channel_id: ID,
    pub content: String,
}

// PROBLEM:
// how to handle anonymous messaging?
// - instead of mapping to only permissions for channel members, map to a channel member object which
// includes an optional "display" of "role" or "mysterious", etc...

// handle permissions, repeating, and relaying

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
        if let Some(member_data) = member {
            if !member_data.perms.contains(ChannelPermission::Send) {
                return Err(ActionError::InsufficientPermissions);
            }
        } else {
            return Err(ActionError::NotAChannelMember);
        }

        // repeating
        for player_id in channel.members.keys() {
            // TODO:
            // relay the message to everyone in the channel (including the sender) (frontend will likely handle the view context)
        }

        // relays
        // TODO:
        // loop through all bugs and relay the messages if the channel is loggable and the bug
        // applies to the person who sent the message

        Ok(ActionResponse::SendMessage(SendMessageResponse {}))
    }
}
