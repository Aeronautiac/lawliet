/*
* SYSTEM ACTION
* Create a channel
*/

use crate::{
    ID,
    action::{ActionInterface, ActionResponse},
    channel::Channel,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CreateChannelResponse {
    pub id: ID,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CreateChannel {
    pub loggable: bool,
}

impl ActionInterface for CreateChannel {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.require_system()?;

        let id = if mutate {
            eng.world.add_channel(Channel::new(self.loggable))
        } else {
            0
        };

        Ok(ActionResponse::CreateChannel(CreateChannelResponse { id }))
    }
}
