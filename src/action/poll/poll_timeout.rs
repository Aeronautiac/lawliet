/*
* SYSTEM ACTION
* Handle a poll timeout
* (try to resolve the poll, if it accepts, execute, else, just delete it)
*/

use crate::action::{ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PollTimeoutResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PollTimeout {}

impl ActionInterface for PollTimeout {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> ActionResult {
        actor.require_system()?;

        Ok(ActionResponse::PollTimeout(PollTimeoutResponse {}))
    }
}
