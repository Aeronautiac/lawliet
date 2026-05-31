/*
* SYSTEM ACTION
* Archive (disable) a bug. Bugs are never destroyed.
*/

use crate::{
    ID,
    action::{ActionInterface, ActionResponse},
    helpers::get_bug_mut,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ArchiveBugResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ArchiveBug {
    pub bug_id: ID,
}

impl ActionInterface for ArchiveBug {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        _ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        _version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.require_system()?;
        let bug = get_bug_mut(eng, self.bug_id)?;
        if mutate {
            bug.enabled = false;
        }
        Ok(ActionResponse::ArchiveBug(ArchiveBugResponse {}))
    }
}
