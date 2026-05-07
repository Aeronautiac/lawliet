/*
* SYSTEM ACTION
* Remove a player from an organization
*/

use crate::{
    ID,
    action::{ActionError, ActionInterface, ActionResponse},
    actor::{ActorLink, ActorLinkType},
    helpers::{get_actor, get_actor_mut, get_org_mut},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct RemoveFromOrgResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct RemoveFromOrg {
    pub actor_id: ID,
    pub org_id: ID,
}

impl ActionInterface for RemoveFromOrg {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.require_system()?;
        get_actor(eng, self.actor_id)?;

        let org = get_org_mut(eng, self.org_id)?;
        if !org.has_member(self.actor_id) {
            return Err(ActionError::PlayerNotInOrg);
        }

        if mutate {
            org.remove_member(self.actor_id);
            let actor = get_actor_mut(eng, self.actor_id)?;
            actor.sever_link(ActorLink {
                link_type: ActorLinkType::Passive,
                link_dest: self.org_id,
            });
        }

        Ok(ActionResponse::RemoveFromOrg(RemoveFromOrgResponse {}))
    }
}
