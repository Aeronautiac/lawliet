/*
* PLAYER ACTION
* Leave a lounge
*/

use crate::{
    ID,
    action::{ActionInterface, ActionResponse},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LeaveLoungeResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LeaveLounge {
    pub lounge_id: ID,
}

impl ActionInterface for LeaveLounge {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.player_only()?;

        Ok(ActionResponse::LeaveLounge(LeaveLoungeResponse {}))
    }
}
