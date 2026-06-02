/*
* SYSTEM ACTION
* Destroy a channel and remove it from the world.
* Callers are responsible for cleaning up any wrapper objects (lounges, groupchats, notebooks,
* world channels) that reference this channel before calling this action.
* TODO: emit frontend command when command protocol is implemented
*/

use crate::{
    ID,
    action::{ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult},
    helpers::get_channel,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DestroyChannelResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DestroyChannel {
    pub channel_id: ID,
}

impl ActionInterface for DestroyChannel {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        _ctx: &mut ActionContext,
        actor: &ActionActor,
        _version: crate::common::Version,
        mutate: bool,
    ) -> ActionResult {
        actor.require_system()?;
        get_channel(eng, self.channel_id)?;

        if mutate {
            eng.world.remove_channel(self.channel_id);
        }

        Ok(ActionResponse::DestroyChannel(DestroyChannelResponse {}))
    }
}
