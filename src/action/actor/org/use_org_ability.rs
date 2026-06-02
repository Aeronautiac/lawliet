/*
* PLAYER ONLY
* Try to use an organization ability
* This action wraps SystemUseOrgAbility
*/

use crate::{
    ability::AbilityBehaviour,
    action::{
        Action, ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult,
        actor::org::system_use_org_ability::SystemUseOrgAbility,
    },
    common::{AbilityKey, ActorKey, PollKey},
    helpers::actor_id,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UseOrgAbilityResponse {
    pub poll_id: Option<PollKey>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UseOrgAbility {
    pub org_id: ActorKey,
    pub ability_id: AbilityKey,
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

        let response = Action::SystemUseOrgAbility(SystemUseOrgAbility {
            org_id: self.org_id,
            user_id: actor_id(actor).unwrap(),
            ability_id: self.ability_id,
            ability_args: self.ability_args.clone(),
            dont_vote: false,
        })
        .handle(eng, ctx, &ActionActor::System, version, mutate)?;
        let ActionResponse::SystemUseOrgAbility(use_response) = response else {
            unreachable!()
        };
        let poll_id = use_response.poll_id;

        Ok(ActionResponse::UseOrgAbility(UseOrgAbilityResponse {
            poll_id,
        }))
    }
}
