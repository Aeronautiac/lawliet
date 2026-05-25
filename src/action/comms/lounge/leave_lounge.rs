/*
* PLAYER ACTION
* Leave a lounge
*/

use crate::{
    ID,
    action::{
        Action, ActionInterface, ActionResponse,
        comms::lounge::remove_from_lounge::RemoveFromLounge,
    },
    helpers::player_id,
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
        let id = player_id(actor).expect("expected valid player id");

        Action::RemoveFromLounge(RemoveFromLounge {
            lounge_id: self.lounge_id,
            player_id: id,
        })
        .handle(
            eng,
            ctx,
            &crate::action::ActionActor::System,
            version,
            mutate,
        )?;

        Ok(ActionResponse::LeaveLounge(LeaveLoungeResponse {}))
    }
}
