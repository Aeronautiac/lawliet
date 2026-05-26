/*
* SYSTEM ACTION
* Set the owner of a group chat
*/

use crate::{
    ID,
    action::{ActionInterface, ActionResponse},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SetGroupchatOwnerResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SetGroupchatOwner {
    pub groupchat_id: ID,
    pub owner: Option<ID>,
}

impl ActionInterface for SetGroupchatOwner {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.require_system()?;

        Ok(ActionResponse::SetGroupchatOwner(
            SetGroupchatOwnerResponse {},
        ))
    }
}
