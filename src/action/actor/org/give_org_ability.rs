/*
* SYSTEM ACTION
* Give an ability to an org including the org ability metadata
*/

use crate::{
    ID,
    action::{Action, ActionInterface, ActionResponse, ability::give_ability::GiveAbility},
    actor::organization::OrgAbility,
    helpers::{get_org, get_org_mut},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct GiveOrgAbilityResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct GiveOrgAbility {
    pub org_id: ID,
    pub ability_id: ID,
    pub settings: OrgAbility,
}

// TODO:
// new action for modifying owned ability metadata

impl ActionInterface for GiveOrgAbility {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.admin_or_system()?;
        get_org(eng, self.org_id)?;

        Action::GiveAbility(GiveAbility {
            ability_id: self.ability_id,
            actor_id: self.org_id,
            volatile: false,
        })
        .handle(eng, ctx, actor, version, mutate)?;

        let org = get_org_mut(eng, self.org_id)?;
        if mutate {
            org.add_ability(self.ability_id, self.settings.clone());
        }

        Ok(ActionResponse::GiveOrgAbility(GiveOrgAbilityResponse {}))
    }
}
