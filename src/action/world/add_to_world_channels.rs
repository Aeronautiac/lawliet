/*
* SYSTEM ACTION
* Add a player to every world channel with no permissions, then evaluate their starting
* permissions via UpdateWorldChannelPerms.
*/

use indexmap::indexset;

use crate::{
    action::{
        Action, ActionInterface, ActionResponse,
        comms::channel::set_member::SetMember,
        world::update_world_channel_perms::UpdateWorldChannelPerms,
    },
    channel::{ChannelMember, ChannelPermissions, SenderDisplay},
    common::{ActorKey, ChannelKey},
    helpers::get_player,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct AddToWorldChannelsResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct AddToWorldChannels {
    pub player_id: ActorKey,
}

impl ActionInterface for AddToWorldChannels {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.admin_or_system()?;
        get_player(eng, self.player_id)?;

        let channel_ids: Vec<ChannelKey> = eng
            .world
            .world_channel_map
            .values()
            .copied()
            .collect();

        for channel_id in channel_ids {
            Action::SetMember(SetMember {
                player_id: self.player_id,
                channel_id,
                settings: Some(ChannelMember {
                    perms: ChannelPermissions::EMPTY,
                    displays: indexset![SenderDisplay::Raw(self.player_id)],
                }),
            })
            .handle(eng, ctx, actor, version, mutate)?;
        }

        Action::UpdateWorldChannelPerms(UpdateWorldChannelPerms {
            player_id: self.player_id,
        })
        .handle(eng, ctx, actor, version, mutate)?;

        Ok(ActionResponse::AddToWorldChannels(
            AddToWorldChannelsResponse {},
        ))
    }
}
