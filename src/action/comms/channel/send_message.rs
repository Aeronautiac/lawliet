/*
* PLAYER ACTION
* Send a message to a channel
*/

use crate::{
    ID,
    action::{ActionError, ActionInterface, ActionResponse},
    channel::{ChannelPermission, SenderDisplay},
    command::Command,
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
        _version: crate::common::Version,
        _mutate: bool,
    ) -> crate::action::ActionResult {
        actor.player_only()?;
        let id = player_id(actor).expect("expected valid player id");

        let loggable = {
            let channel = get_channel(eng, self.channel_id)?;
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
            channel.loggable
        };

        // this will tell the frontend to show the message to everyone who has view permissions for
        // this channel
        ctx.push_cmd(
            Command::AddMessage {
                content: self.content.clone(),
                channel_id: self.channel_id,
                sender_display: self.display,
            },
            None,
            eng.time,
        );

        // relay to all active bugs targeting this player if the channel is loggable
        if loggable {
            let bug_ids: Vec<ID> = eng
                .world
                .get_player(id)
                .expect("expected valid player")
                .bugs
                .iter()
                .copied()
                .collect();
            for bug_id in bug_ids {
                let bug = eng.world.get_bug(bug_id).expect("expected valid bug");
                if bug.enabled {
                    let _ = bug_id;
                    // TODO: push BugRelay command once the bug log protocol is implemented
                }
            }
        }

        Ok(ActionResponse::SendMessage(SendMessageResponse {}))
    }
}
