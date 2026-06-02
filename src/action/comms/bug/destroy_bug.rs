/*
* SYSTEM ACTION
* TODO: implement
*/

use crate::{
    ID,
    action::{ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DestroyBugResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DestroyBug {
    pub bug_id: ID,
}

impl ActionInterface for DestroyBug {
    fn handle(
        &mut self,
        _eng: &mut crate::engine::Engine,
        _ctx: &mut ActionContext,
        actor: &ActionActor,
        _version: crate::common::Version,
        _mutate: bool,
    ) -> ActionResult {
        actor.admin_or_system()?;
        // TODO: implement
        Ok(ActionResponse::DestroyBug(DestroyBugResponse {}))
    }
}
