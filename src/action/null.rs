use crate::action::{ActionInterface, ActionResponse};

#[derive(PartialEq, Eq, Clone)]
pub struct NullResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct Null {}

impl ActionInterface for Null {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut super::ActionContext,
        actor: &super::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> super::ActionResult {
        Ok(ActionResponse::Null(NullResponse {}))
    }
}
