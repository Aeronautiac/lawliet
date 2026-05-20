/*
* PLAYER ONLY
* Try to use an organization ability
* This action wraps SystemUseOrgAbility
*/

use crate::{
    ID,
    ability::AbilityBehaviour,
    action::{
        Action, ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult,
        actor::org::system_use_org_ability::SystemUseOrgAbility,
    },
    helpers::actor_id,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UseOrgAbilityResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UseOrgAbility {
    pub org_id: ID,
    pub ability_id: ID,
    pub ability_args: AbilityBehaviour,
}

impl ActionInterface for UseOrgAbility {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> ActionResult {
        actor.player_only()?;

        Action::SystemUseOrgAbility(SystemUseOrgAbility {
            org_id: self.org_id,
            user_id: actor_id(actor).unwrap(),
            ability_id: self.ability_id,
            ability_args: self.ability_args.clone(),
            dont_vote: false,
        })
        .handle(eng, ctx, actor, version, mutate)?;

        Ok(ActionResponse::UseOrgAbility(UseOrgAbilityResponse {}))
    }
}
