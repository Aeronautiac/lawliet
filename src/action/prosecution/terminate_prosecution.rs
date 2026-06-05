/*
* Authoritative Action
* Remove custody, delete any channels (archive true), cancel jobs
*/

// prosecutions are archived on termination.

use crate::action::ActionInterface;

pub struct TerminateProsecutionResult {}

pub struct TerminateProsecution {
    //pub prosecution_id: ProsecutionKey
}

impl ActionInterface for TerminateProsecution {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.admin_or_system()?;

        unimplemented!()
    }
}
