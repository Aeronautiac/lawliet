/*
* PLAYER & ORG ONLY
* Use an ability
*/

use crate::{
    ID,
    ability::{AbilityBehaviour, AbilityInterface, AbilityResponse},
    action::{
        ActionActor, ActionContext, ActionError, ActionInterface, ActionResponse, ActionResult,
    },
    actor::restriction::Restriction,
    chargepool::PoolLinkType,
    helpers::{
        actor_id, get_ability, get_ability_config, get_ability_mut, get_actor, get_charge_pool_mut,
    },
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UseAbilityResponse(AbilityResponse);

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UseAbility {
    pub ability_id: ID,
    pub ability_args: AbilityBehaviour,
}

impl ActionInterface for UseAbility {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> ActionResult {
        actor.require_not_system()?;
        let actor_id = actor_id(actor).unwrap();
        let config = get_ability_config(eng, self.ability_id)?;
        let req_presence = config.require_presence;

        let actor_data = get_actor(eng, actor_id)?;
        if req_presence && actor_data.has_restriction(Restriction::Presence) {
            return Err(ActionError::AbilityCategoryBlocked);
        }

        let ability = get_ability(eng, self.ability_id)?;
        if Some(self.ability_id) != ability.ownership_struct.owner {
            return Err(ActionError::AbilityNotOwned);
        }
        if ability.ability_name != self.ability_args.ability_name() {
            return Err(ActionError::AbilityMismatch);
        }

        // its safe to modify (with mutate flag) before all checks have seemingly been performed
        // base on code ordering because condition should only fail during a validation pass, never during a mutation
        // pass. if it does for some reason, then the engine crashes and there is no risk of invalid
        // state.
        let ability = get_ability_mut(eng, self.ability_id)?;
        let links = ability.pool_links.clone();
        let mut pool_condition = false;
        for link in &links {
            let pool = get_charge_pool_mut(eng, link.link.link_dest)?;
            match link.link.link_type {
                PoolLinkType::Limit => {
                    if !pool.can_use_charges(link.link.weight) {
                        return Err(ActionError::AbilityNotEnoughCharges);
                    }
                }
                PoolLinkType::Pool => {
                    if pool.can_use_charges(link.link.weight) {
                        pool_condition = true;
                    }
                }
            }
            if mutate && pool.can_use_charges(link.link.weight) {
                pool.use_charges(link.link.weight);
            }
        }
        if !pool_condition {
            return Err(ActionError::AbilityNotEnoughCharges);
        }

        let response =
            self.ability_args
                .handle(eng, ctx, actor, self.ability_id, version, mutate)?;

        Ok(ActionResponse::UseAbility(UseAbilityResponse(response)))
    }
}
