/*
* SYSTEM ACTION
* Schedule a revive action
*/

use crate::{
    Timestamp,
    action::{
        Action, ActionActor, ActionContext, ActionInterface, ActionRequest, ActionResponse,
        ActionResult, require_time_not_passed, revive::Revive,
    },
    common::Version,
    engine::Engine,
};

#[derive(PartialEq, Eq, Clone)]
pub struct ScheduleReviveResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct ScheduleRevive {
    pub timestamp: Timestamp,
    pub revive: Revive,
}

impl ActionInterface for ScheduleRevive {
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
                actor: ActionActor::System,
                timestamp: self.timestamp,
                payload: Action::Revive(self.revive.clone()),
            })
        }

        Ok(ActionResponse::ScheduleRevive(ScheduleReviveResponse {}))
    }
}
