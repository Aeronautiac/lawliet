/*
* SYSTEM ACTION
* A kill wrapper used to differentiate between notebook scheduled kill jobs and host/system
* scheduled jobs
* This can potentially hold metadata in the future such as the ID for the notebook which scheduled
* the job
*/

use crate::action::{Action, ActionInterface, ActionResponse, kill::Kill};

#[derive(PartialEq, Eq, Clone)]
pub struct NotebookScheduledKillResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct NotebookScheduledKill {
    pub kill: Kill,
}

impl ActionInterface for NotebookScheduledKill {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut super::ActionContext,
        actor: &super::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> super::ActionResult {
        actor.require_system()?;

        Action::Kill(self.kill.clone()).handle(eng, ctx, actor, version, mutate)?;

        Ok(ActionResponse::NotebookScheduledKill(
            NotebookScheduledKillResponse {},
        ))
    }
}
