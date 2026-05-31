/*
* SYSTEM ACTION
* Set or clear a player's per-channel override (default and/or force permissions) for a
* world channel, then re-evaluate their effective permissions.
*/

use crate::{
    ID,
    action::{
        Action, ActionInterface, ActionResponse,
        world::update_world_channel_perms::UpdateWorldChannelPerms,
    },
    actor::player::WorldChannelOverride,
    config::world::WorldChannelName,
    helpers::get_player_mut,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SetWorldChannelOverrideResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SetWorldChannelOverride {
    pub player_id: ID,
    pub channel_name: WorldChannelName,
    pub override_data: Option<WorldChannelOverride>, // None clears the override
}

impl ActionInterface for SetWorldChannelOverride {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.require_system()?;

        let player = get_player_mut(eng, self.player_id)?;
        if mutate {
            match &self.override_data {
                Some(over) => {
                    player
                        .world_channel_overrides
                        .insert(self.channel_name, over.clone());
                }
                None => {
                    player
                        .world_channel_overrides
                        .swap_remove(&self.channel_name);
                }
            }
        }

        Action::UpdateWorldChannelPerms(UpdateWorldChannelPerms {
            player_id: self.player_id,
        })
        .handle(eng, ctx, actor, version, mutate)?;

        Ok(ActionResponse::SetWorldChannelOverride(
            SetWorldChannelOverrideResponse {},
        ))
    }
}
