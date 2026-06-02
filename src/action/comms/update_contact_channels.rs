/*
* SYSTEM ACTION
* Update a player's contact channel (groupchats and lounges) permissions based on current state.
*/

use crate::{
    action::{ActionInterface, ActionResponse},
    actor::modifier::Modifier,
    channel::{ChannelPermission, ChannelPermissions},
    common::ActorKey,
    helpers::{get_actor, get_channel_mut, get_gc, get_lounge, get_player},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UpdateContactChannelsResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UpdateContactChannels {
    pub player_id: ActorKey,
}

impl ActionInterface for UpdateContactChannels {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.admin_or_system()?;

        let actor_data = get_actor(eng, self.player_id)?;
        let no_contact = actor_data.has_modifier(Modifier::NoContact);

        let player_data = get_player(eng, self.player_id)?;
        let lounges = player_data.lounges.clone();
        let gcs = player_data.groupchats.clone();

        for lounge_id in lounges {
            let lounge = get_lounge(eng, lounge_id)?;
            let channel = get_channel_mut(eng, lounge.channel_id)?;
            let mut member_settings = channel
                .get_member(self.player_id)
                .expect("expected player to be in a lounge within their lounge cache")
                .clone();
            if mutate {
                if no_contact {
                    member_settings.perms = ChannelPermissions::EMPTY;
                } else {
                    member_settings.perms = ChannelPermission::Send | ChannelPermission::View;
                }
                channel.set_member(self.player_id, Some(member_settings));
            }
        }

        for gc_id in gcs {
            let gc = get_gc(eng, gc_id)?;
            let channel = get_channel_mut(eng, gc.channel_id)?;
            let mut member_settings = channel
                .get_member(self.player_id)
                .expect("expected player to be in a gc within their gc cache")
                .clone();
            if mutate {
                if no_contact {
                    member_settings.perms = ChannelPermissions::EMPTY;
                } else {
                    member_settings.perms = ChannelPermission::Send | ChannelPermission::View;
                }
                channel.set_member(self.player_id, Some(member_settings));
            }
        }

        Ok(ActionResponse::UpdateContactChannels(
            UpdateContactChannelsResponse {},
        ))
    }
}
