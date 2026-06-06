/*
* SYSTEM ACTION
* Archive (disable) a bug.
*/

use crate::{
    action::{ActionInterface, ActionResponse},
    command::Command,
    common::BugKey,
    helpers::get_bug_mut,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ArchiveBugResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ArchiveBug {
    pub bug_id: BugKey,
}

impl ActionInterface for ArchiveBug {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        _version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.admin_or_system()?;

        let bug = get_bug_mut(eng, self.bug_id)?;
        if mutate {
            bug.enabled = false;
        }

        ctx.push_cmd(
            Command::ArchiveBug {
                bug_key: self.bug_id,
            },
            None,
            eng.time,
        );

        Ok(ActionResponse::ArchiveBug(ArchiveBugResponse {}))
    }
}
