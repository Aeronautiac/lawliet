use crate::action::{ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct NullResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Null {}

impl ActionInterface for Null {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> ActionResult {
        Ok(ActionResponse::Null(NullResponse {}))
    }
}
