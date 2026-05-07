/*
* SYSTEM ACTION
* Schedule a kill action
*/

use crate::{
    Time,
    action::{
        Action, ActionActor, ActionContext, ActionInterface, ActionRequest, ActionResponse,
        ActionResult, actor::player::kill::Kill,
        notebook::notebook_scheduled_kill::NotebookScheduledKill,
    },
    common::Version,
    engine::Engine,
    helpers::require_time_not_passed,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ScheduleKillResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ScheduleKill {
    pub timestamp: Time,
    pub kill: Kill,
    pub notebook_scheduled: bool,
}

impl ActionInterface for ScheduleKill {
    fn handle(
        &mut self,
        eng: &mut Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: Version,
        mutate: bool,
    ) -> ActionResult {
        actor.require_system()?;
        require_time_not_passed(eng, self.timestamp)?;

        if mutate {
            if self.notebook_scheduled {
                eng.schedule(ActionRequest {
                    actor: ActionActor::System,
                    timestamp: self.timestamp,
                    payload: Action::NotebookScheduledKill(NotebookScheduledKill {
                        kill: self.kill.clone(),
                    }),
                })
            } else {
                eng.schedule(ActionRequest {
                    actor: ActionActor::System,
                    timestamp: self.timestamp,
                    payload: Action::Kill(self.kill.clone()),
                })
            }
        }

        Ok(ActionResponse::ScheduleKill(ScheduleKillResponse {}))
    }
}
