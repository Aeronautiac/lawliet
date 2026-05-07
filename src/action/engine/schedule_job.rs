/*
* SYSTEM ACTION
* Schedule a job
*/

use crate::{
    Time,
    action::{
        Action, ActionActor, ActionContext, ActionInterface, ActionRequest, ActionResponse,
        ActionResult,
    },
    common::Version,
    engine::Engine,
    helpers::require_time_not_passed,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ScheduleJobResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ScheduleJob {
    pub timestamp: Time,
    pub payload: Box<Action>,
}

impl ActionInterface for ScheduleJob {
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
            eng.schedule(ActionRequest {
                actor: actor.clone(),
                timestamp: self.timestamp,
                payload: *self.payload.clone(),
            });
        }

        Ok(ActionResponse::ScheduleJob(ScheduleJobResponse {}))
    }
}
