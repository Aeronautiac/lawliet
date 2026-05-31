/*
* SYSTEM ACTION
* Re-evaluate a player's permissions in every world channel based on their current modifiers
* and any per-channel overrides. Called when modifiers change.
* Skips channels the player is not a member of (e.g. explicitly removed by a host).
*/

use crate::{
    ID,
    action::{
        Action, ActionInterface, ActionResponse,
        comms::channel::set_member::SetMember,
    },
    channel::{ChannelMember, ChannelPermission, ChannelPermissions, SenderDisplay},
    helpers::{get_actor, get_player},
};

use indexmap::IndexSet;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UpdateWorldChannelPermsResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UpdateWorldChannelPerms {
    pub player_id: ID,
}

impl ActionInterface for UpdateWorldChannelPerms {
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

        let player_modifiers = get_actor(eng, self.player_id)?.modifiers();
        let updates: Vec<(ID, ChannelPermissions, IndexSet<SenderDisplay>)> = {
            let player = get_player(eng, self.player_id)?;
            eng.world
                .world_channel_map
                .iter()
                .filter_map(|(name, &channel_id)| {
                    let config = eng.config.world_config.world_channels.get(name)?;
                    let over = player.world_channel_overrides.get(name);

                    let base = over.map_or(config.default_perms, |o| o.default_perms);
                    let force = over.map_or(ChannelPermissions::EMPTY, |o| o.force_perms);

                    let mut blocked = ChannelPermissions::EMPTY;
                    if !(player_modifiers & config.send_blocking).is_empty() {
                        blocked |= ChannelPermission::Send;
                    }
                    if !(player_modifiers & config.view_blocking).is_empty() {
                        blocked |= ChannelPermission::View;
                    }

                    let effective = (base & !blocked) | force;
                    let displays = eng
                        .world
                        .get_channel(channel_id)?
                        .get_member(self.player_id)?
                        .displays
                        .clone();

                    Some((channel_id, effective, displays))
                })
                .collect()
        };

        for (channel_id, effective_perms, displays) in updates {
            Action::SetMember(SetMember {
                player_id: self.player_id,
                channel_id,
                settings: Some(ChannelMember {
                    perms: effective_perms,
                    displays,
                }),
            })
            .handle(eng, ctx, actor, version, mutate)?;
        }

        Ok(ActionResponse::UpdateWorldChannelPerms(
            UpdateWorldChannelPermsResponse {},
        ))
    }
}
