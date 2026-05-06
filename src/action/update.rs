/*
* SYSTEM ACTION
* Keep game state up to date for anything that is fairly isolated but dependent
* on everything else in game and may in of itself influence game state
*/

use crate::action::{ActionInterface, ActionResponse};

#[derive(PartialEq, Eq, Clone)]
pub struct UpdateResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct Update {}

impl ActionInterface for Update {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut super::ActionContext,
        actor: &super::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> super::ActionResult {
        actor.require_system()?;

        Ok(ActionResponse::Update(UpdateResponse {}))
    }
}
