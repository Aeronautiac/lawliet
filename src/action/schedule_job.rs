/*
* SYSTEM ACTION
* Schedule a job to be executed later, return the sequence number to the caller
*/

use crate::{
    Timestamp,
    action::{
        Action, ActionActor, ActionError, ActionInterface, ActionResponse, kill::Kill,
        revive::Revive,
    },
    common::SequenceNumber,
    engine::Engine,
};

#[derive(PartialEq, Eq, Clone)]
pub struct ScheduleJobResponse {
    seq_num: SequenceNumber,
}

#[derive(PartialEq, Eq, Clone)]
pub enum Schedulable {
    Kill(Kill),
    Revive(Revive),
}

#[derive(PartialEq, Eq, Clone)]
pub struct ScheduleJob {
    pub timestamp: Timestamp,
    pub actor: ActionActor,
    pub action: Schedulable,
}

impl ActionInterface for ScheduleJob {
    fn validate(&self, eng: &Engine, actor: &ActionActor) -> Result<(), ActionError> {
        actor.require_system()?;
        if self.timestamp < eng.time {
            return Err(ActionError::TimestampAlreadyPassed);
        }
        Ok(())
    }

    fn execute(self, eng: &mut Engine, actor: &ActionActor) -> ActionResponse {
        unimplemented!()
    }
}
