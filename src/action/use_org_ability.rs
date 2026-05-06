/*
* PLAYER ONLY
* Try to use an organization ability
* This action wraps UseAbility
*/

use crate::{
    ID,
    ability::AbilityBehaviour,
    action::{
        Action, ActionActor, ActionError, ActionInterface, ActionResponse, use_ability::UseAbility,
    },
    actor::organization::OrgAbilityPolicy,
    helpers::get_org,
};

#[derive(PartialEq, Eq, Clone)]
pub struct UseOrgAbilityResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct UseOrgAbility {
    pub org_id: ID,
    pub ability_id: ID,
    pub ability_args: AbilityBehaviour,
}

impl ActionInterface for UseOrgAbility {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut super::ActionContext,
        actor: &super::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> super::ActionResult {
        actor.player_only()?;

        if let Ok(org_data) = get_org(eng, self.org_id) {
            let ActionActor::Organization(org_info) = actor else {
                unreachable!();
            };
            let player_id = org_info.player_id;
            let ability_policies = org_data.abilities.get(&self.ability_id).unwrap();
            if ability_policies.contains(OrgAbilityPolicy::RequireLeader) {
                let Some(leadership_struct) = &org_data.leadership_struct else {
                    unreachable!(); // there must be a leadership struct if the ability requires a leader
                };
                if leadership_struct.leader != player_id {
                    return Err(ActionError::PlayerIsNotLeader);
                }
            }
            if ability_policies.contains(OrgAbilityPolicy::RequireVote) {
                // TODO:
                // - Create a poll
            } else {
                Action::UseAbility(UseAbility {
                    ability_id: self.ability_id,
                    ability_args: self.ability_args.clone(),
                })
                .handle(eng, ctx, actor, version, mutate)?;
            }
        }

        Ok(ActionResponse::UseOrgAbility(UseOrgAbilityResponse {}))
    }
}
