/*
* SYSTEM ACTION
* Change the leader of an org
* The new leader can either be None or Some(leader_id)
* The new leader must already be in the org
*/

// notify existing leaders that leadership has changed

use crate::{
    ID,
    action::{ActionError, ActionInterface, ActionResponse},
    helpers::{get_actor, get_org_mut},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ChangeOrgLeaderResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ChangeOrgLeader {
    pub org_id: ID,
    pub new_leader: ID,
}

// TODO:
// possibly enforce the person being a member

impl ActionInterface for ChangeOrgLeader {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.require_system()?;
        get_actor(eng, self.new_leader)?;

        let org = get_org_mut(eng, self.org_id)?;
        if let Some(leadership_struct) = &mut org.leadership_struct {
            if Some(self.new_leader) == leadership_struct.leader {
                return Err(ActionError::AlreadyLeader);
            }
            if let Some(leader) = leadership_struct.leader {
                // TODO:
                // alert them of leadership change
            }
            leadership_struct.leader = Some(self.new_leader);
        } else {
            return Err(ActionError::OrgDoesntHaveLeadership);
        }

        Ok(ActionResponse::ChangeOrgLeader(ChangeOrgLeaderResponse {}))
    }
}
