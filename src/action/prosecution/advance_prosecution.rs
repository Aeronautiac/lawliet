/*
* SYSTEM / ADMIN ACTION
* Drive the prosecution state machine forward by one phase or subphase.
*
* Called by:
* - custody timeout job (system)
* - both ready flags set in Custody (system, after SignalReady)
* - first message sent by the active side during a Grace subphase (system, from SendMessage)
* - presentation/debate timeout job (system)
* - both done flags set in Debate (system, after SignalDone)
* - host manually advancing a non-autonomous prosecution (admin)
*
* All players with presence are added to the trial channel with view permissions (granted via
* deferred commands). Send permissions are restricted to the active side per subphase and are
* set directly — the trial terminates on NoPresence, so there is no case where a player loses
* and then regains send permissions mid-trial.
*
* Transitions:
*   Custody → Trial:
*     cancel custody timeout job
*     create trial channel (loggable)
*     grant view to all present players via deferred commands
*     grant send to prosecutor side only
*     schedule prosecutor grace timeout → timeout_job_id
*     phase = Trial { Prosecutor(Grace), timeout_job_id }
*
*   Trial Prosecutor(Grace) → Prosecutor(Presentation):
*     cancel grace job, schedule presentation timeout
*     (send already restricted to prosecutor side)
*
*   Trial Prosecutor(Presentation) → Defense(Grace):
*     restrict send to defense side
*     schedule defense grace timeout
*
*   Trial Defense(Grace) → Defense(Presentation):
*     cancel grace job, schedule presentation timeout
*     (send already restricted to defense side)
*
*   Trial Defense(Presentation) → Debate:
*     grant send to both sides
*     schedule debate timeout
*     phase = Trial { Debate { prosecutor_done: false, defense_done: false }, timeout_job_id }
*
*   Trial Debate → Voting (timer expired or both done):
*     revoke all send permissions in trial channel
*     create poll in trial channel
*     phase = Voting { poll_id }
*
* TODO: commands
*/

use crate::{
    action::{ActionInterface, ActionResult},
    common::ProsecutionKey,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct AdvanceProsecutionResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct AdvanceProsecution {
    pub prosecution_id: ProsecutionKey,
}

impl ActionInterface for AdvanceProsecution {
    fn handle(
        &mut self,
        _eng: &mut crate::engine::Engine,
        _ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        _version: crate::common::Version,
        _mutate: bool,
    ) -> ActionResult {
        actor.admin_or_system()?;
        unimplemented!()
    }
}
