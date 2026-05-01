/*
* SYSTEM ACTION
* Schedule a kill action
* The reason there isn't a generic schedule job action is because the compiler cannot know the size
* of the action beforehand. It is technically possible (in the view of the compiler) for a scheduling
* action to schedule another scheduling action. While something like a box could be used, it is better
* for performance to simply create specific scheduling actions (avoids pointer chasing).
*/

use crate::{
    Timestamp,
    action::{
        Action, ActionActor, ActionContext, ActionInterface, ActionRequest, ActionResponse,
        ActionResult, kill::Kill,
    },
    common::Version,
    engine::Engine,
    helpers::require_time_not_passed,
};

#[derive(PartialEq, Eq, Clone)]
pub struct ScheduleKillResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct ScheduleKill {
    pub timestamp: Timestamp,
    pub kill: Kill,
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
            eng.schedule(ActionRequest {
                actor: ActionActor::System,
                timestamp: self.timestamp,
                payload: Action::Kill(self.kill.clone()),
            })
        }

        Ok(ActionResponse::ScheduleKill(ScheduleKillResponse {}))
    }
}
