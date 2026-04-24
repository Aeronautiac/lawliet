/*
* SYSTEM ACTION
* Schedule a kill action
* The reason there isn't a generic schedule job action is because the compiler cannot know the size
* of the action beforehand. It is technically possible (in the view of the compiler) for a scheduling
* action to schedule another scheduling action. While something like a box could be used, it is better
* for the cache to simply create specific scheduling actions.
*/

use crate::{
    Timestamp,
    action::{
        Action, ActionActor, ActionInterface, ActionRequest, ActionResponse, ActionResult,
        ResponseData, kill::Kill, require_time_not_passed,
    },
    common::Version,
    engine::Engine,
};

#[derive(PartialEq, Eq, Clone)]
pub struct ScheduleKillResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct ScheduleKill {
    timestamp: Timestamp,
    kill: Kill,
}

impl ActionInterface for ScheduleKill {
    fn handle(
        &mut self,
        eng: &mut Engine,
        actor: &ActionActor,
        _: Version,
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

        Ok(ActionResponse {
            commands: vec![],
            next_actions: vec![],
            data: ResponseData::ScheduleKill(ScheduleKillResponse {}),
        })
    }
}
