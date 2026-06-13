/*
System Action
Update an actor's access to the prison channel based on their current state
Incarcerated? Send + View
Not incarcerated? Empty.
*/

use crate::{
    ActorKey,
    action::{
        Action, ActionActor, ActionInterface, ActionResponse,
        world::set_world_channel_override::SetWorldChannelOverride,
    },
    actor::{
        player::{OverrideSource, WorldChannelOverride},
        state::State,
    },
    channel::{ChannelPermission, ChannelPermissions},
    config::world::WorldChannelName,
    helpers::get_actor,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UpdatePrisonChannelResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UpdatePrisonChannel {
    pub actor_id: ActorKey,
}

impl ActionInterface for UpdatePrisonChannel {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.admin_or_system()?;
        let actor_data = get_actor(eng, self.actor_id)?;

        let over: Option<WorldChannelOverride> = if actor_data.has_state(State::Incarcerated) {
            Some(WorldChannelOverride {
                default_perms: ChannelPermission::Send | ChannelPermission::View,
                force_perms: ChannelPermissions::EMPTY,
            })
        } else {
            None
        };

        Action::SetWorldChannelOverride(SetWorldChannelOverride {
            channel_name: WorldChannelName::Prison,
            override_data: over,
            player_id: self.actor_id,
            source: OverrideSource::Incarceration,
            priority: 0,
        })
        .handle(eng, ctx, &ActionActor::System, version, mutate)?;

        Ok(ActionResponse::UpdatePrisonChannel(
            UpdatePrisonChannelResponse {},
        ))
    }
}
