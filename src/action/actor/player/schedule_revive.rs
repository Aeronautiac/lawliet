/*
* SYSTEM ACTION
* Schedule a revive action
*/

use crate::{
    Time,
    action::{
        Action, ActionActor, ActionContext, ActionInterface, ActionRequest, ActionResponse,
        ActionResult, actor::player::revive::Revive,
    },
    common::Version,
    engine::Engine,
    helpers::require_time_not_passed,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ScheduleReviveResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ScheduleRevive {
    pub timestamp: Time,
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
