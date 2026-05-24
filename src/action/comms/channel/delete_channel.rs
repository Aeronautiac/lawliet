/*
* SYSTEM ACTION
* Delete a channel
*/

use crate::{
    ID,
    action::{ActionInterface, ActionResponse},
    helpers::get_channel,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DeleteChannelResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DeleteChannel {
    pub channel_id: ID,
}

impl ActionInterface for DeleteChannel {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.require_system()?;
        get_channel(eng, self.channel_id)?;

        if mutate {
            eng.world.remove_channel(self.channel_id);
        }

        Ok(ActionResponse::DeleteChannel(DeleteChannelResponse {}))
    }
}
