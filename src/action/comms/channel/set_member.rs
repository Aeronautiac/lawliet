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

        let time = eng.time;
        let channel = get_channel_mut(eng, self.channel_id)?;
        if mutate {
            channel.set_member(self.player_id, self.settings.clone());
        }

        if let Some(member) = &self.settings {
            // note that the player is sent a member display command for their own displays as well
            // this is just more convenient than having to derive it on the frontend + it would require
            // extra backend logic
            for (_, member) in channel.members.iter() {
                for display in member.displays.iter() {
                    ctx.push_cmd(
                        Command::ShowChannelMember {
                            channel_id: self.channel_id,
                            display: *display,
                            channel_perms: member.perms,
                        },
                        Some(self.player_id),
                        time,
                    );
                }
            }

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
            for (_, member) in channel.members.iter() {
                for display in member.displays.iter() {
                    ctx.push_cmd(
                        Command::RemoveChannelMember {
                            channel_id: self.channel_id,
                            display: *display,
                        },
                        Some(self.player_id),
                        time,
                    );
                }
            }

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
