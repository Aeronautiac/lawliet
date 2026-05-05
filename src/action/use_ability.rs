/*
* PLAYER & ORG ONLY
* Use an ability
*/

use crate::{
    ID,
    ability::{AbilityBehaviour, AbilityInterface, AbilityResponse},
    action::{ActionError, ActionInterface, ActionResponse},
    actor::restriction::Restriction,
    chargepool::PoolLinkType,
    config::ability::AbilityCategory,
    helpers::{actor_id, get_ability_config, get_ability_mut, get_actor, get_charge_pool_mut},
};

#[derive(PartialEq, Eq, Clone)]
pub struct UseAbilityResponse(AbilityResponse);

#[derive(PartialEq, Eq, Clone)]
pub struct UseAbility {
    pub ability_id: ID,
    pub ability_args: AbilityBehaviour,
}

impl ActionInterface for UseAbility {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut super::ActionContext,
        actor: &super::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> super::ActionResult {
        actor.require_not_system()?;

        let config = get_ability_config(eng, self.ability_id)?;
        let category = config.category;

        let actor_data = get_actor(eng, actor_id(actor).unwrap())?;
        match category {
            AbilityCategory::Supernatural => {
                if actor_data.has_restriction(Restriction::AbilitiesSupernatural) {
                    return Err(ActionError::AbilityCategoryBlocked);
                }
            }
            AbilityCategory::Physical => {
                if actor_data.has_restriction(Restriction::AbilitiesPhysical) {
                    return Err(ActionError::AbilityCategoryBlocked);
                }
            }
        };

        let ability = get_ability_mut(eng, self.ability_id)?;
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
