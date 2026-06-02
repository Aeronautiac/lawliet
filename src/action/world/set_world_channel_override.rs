/*
* SYSTEM ACTION
* Set or clear a player's per-channel override for a specific source, then re-evaluate
* their effective permissions. Each source may hold at most one override per channel.
*/

use indexmap::IndexMap;

use crate::{
    action::{
        Action, ActionInterface, ActionResponse,
        world::update_world_channel_perms::UpdateWorldChannelPerms,
    },
    actor::player::{OverrideSource, SourcedWorldChannelOverride, WorldChannelOverride},
    common::ActorKey,
    config::world::WorldChannelName,
    helpers::get_player_mut,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SetWorldChannelOverrideResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SetWorldChannelOverride {
    pub player_id: ActorKey,
    pub channel_name: WorldChannelName,
    pub source: OverrideSource,
    pub priority: u8,
    pub override_data: Option<WorldChannelOverride>, // None clears the override for this source
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
        actor.admin_or_system()?;

        let player = get_player_mut(eng, self.player_id)?;
        if mutate {
            match &self.override_data {
                Some(data) => {
                    player
                        .world_channel_overrides
                        .entry(self.channel_name)
                        .or_insert_with(IndexMap::new)
                        .insert(self.source.clone(), SourcedWorldChannelOverride {
                            priority: self.priority,
                            data: data.clone(),
                        });
                }
                None => {
                    if let Some(channel_overrides) = player
                        .world_channel_overrides
                        .get_mut(&self.channel_name)
                    {
                        channel_overrides.swap_remove(&self.source);
                    }
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
