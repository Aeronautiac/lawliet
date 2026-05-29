/*
* SYSTEM ACTION
* Map a player ID to a channel member struct within the channel
*/

use crate::{
    ID,
    action::{ActionInterface, ActionResponse},
    channel::ChannelMember,
    command::Command,
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

impl ActionInterface for SetMember {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        _version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.require_system()?;
        get_player(eng, self.player_id)?;

        let channel = get_channel_mut(eng, self.channel_id)?;
        if mutate {
            channel.set_member(self.player_id, self.settings.clone());
        }

        // TODO:
        // send add/remove member commands to every other member who is currently present (can see
        // the org)

        if let Some(member) = &self.settings {
            ctx.push_cmd(
                Command::UpdateChannelView {
                    channel_id: self.channel_id,
                    displays: member.displays.clone(),
                    perms: member.perms,
                },
                Some(self.player_id),
                eng.time,
            );
        } else {
            ctx.push_cmd(
                Command::RemoveChannel {
                    channel_id: self.channel_id,
                },
                Some(self.player_id),
                eng.time,
            );
        }

        Ok(ActionResponse::SetMember(SetMemberResponse {}))
    }
}
