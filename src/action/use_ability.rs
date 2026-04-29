/*
* PLAYER & ORG ONLY
* Use an ability
*/

use crate::{
    ID,
    ability::{AbilityBehaviour, AbilityInterface, AbilityLinkType, AbilityResponse},
    action::{
        ActionError, ActionInterface, ActionResponse, actor_id, get_ability_config,
        get_ability_mut, get_actor,
    },
    actor::restriction::Restriction,
    config::ability::AbilityCategory,
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
        let reset_time = config.reset_time;
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
        if Some(self.ability_id) != ability.owner {
            return Err(ActionError::AbilityNotOwned);
        }
        if ability.ability_name != self.ability_args.ability_name() {
            return Err(ActionError::AbilityMismatch);
        }

        // its safe to modify (with mutate flag) before all checks have seemingly been performed
        // base on code ordering because condition should only fail during a validation pass, never during a mutation
        // pass. if it does for some reason, then the engine crashes and there is no risk of invalid
        // state.
        if !ability.has_charges() {
            return Err(ActionError::AbilityNotEnoughCharges);
        }
        if mutate {
            ability.on_use(reset_time);
        }

        let links = ability.links.clone();
        let mut pool_condition = false;
        for link in &links {
            let linked_ability = get_ability_mut(eng, link.link_dest)?;
            match link.link_type {
                AbilityLinkType::Limit => {
                    if linked_ability.charges < link.weight {
                        return Err(ActionError::AbilityNotEnoughCharges);
                    }
                }
                AbilityLinkType::Pool => {
                    if linked_ability.charges >= link.weight {
                        pool_condition = true;
                    }
                }
            }
            if mutate && linked_ability.charges >= link.weight {
                linked_ability.charges -= link.weight;
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
