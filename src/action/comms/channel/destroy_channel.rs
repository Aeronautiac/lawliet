/*
* SYSTEM ACTION
* Destroy a channel and remove it from the world.
* Callers are responsible for cleaning up any wrapper objects (lounges, groupchats, notebooks,
* world channels) that reference this channel before calling this action.
*/

use crate::{
    action::{ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult},
    command::Command,
    common::ChannelKey,
    helpers::get_channel,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DestroyChannelResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DestroyChannel {
    pub channel_id: ChannelKey,
}

impl ActionInterface for DestroyChannel {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        _version: crate::common::Version,
        mutate: bool,
    ) -> ActionResult {
        actor.require_system()?;
        get_channel(eng, self.channel_id)?;

        if mutate {
            eng.world.remove_channel(self.channel_id);
        }

        ctx.push_cmd(
            Command::DeleteChannel {
                channel_id: self.channel_id,
            },
            None,
            eng.time,
        );

        Ok(ActionResponse::DestroyChannel(DestroyChannelResponse {}))
    }
}
