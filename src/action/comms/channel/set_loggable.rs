/*
* SYSTEM ACTION
* Set the loggable status of a channel
*/

use crate::{
    ID,
    action::{ActionInterface, ActionResponse},
    helpers::get_channel_mut,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SetLoggableResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SetLoggable {
    pub channel_id: ID,
    pub loggable: bool,
}

impl ActionInterface for SetLoggable {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.admin_or_system()?;

        let channel = get_channel_mut(eng, self.channel_id)?;
        if mutate {
            channel.loggable = self.loggable
        }

        // TODO:
        // host command(s)

        Ok(ActionResponse::SetLoggable(SetLoggableResponse {}))
    }
}
