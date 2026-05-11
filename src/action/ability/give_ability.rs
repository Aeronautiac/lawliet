/*
* SYSTEM ACTION
* Transfer ownership of an ability to a specified actor and then reset links
*/

// TODO:
// Handle organization transfers. Orgs have a map of ability ids to ability metadata.

use crate::{
    ID,
    action::{
        Action, ActionActor, ActionContext, ActionError, ActionInterface, ActionResponse,
        ActionResult, ability::clear_volatile_links::ClearVolatileLinks,
    },
    chargepool::PoolLink,
    config::ability::{AbilityIdentifier, ConfigPoolLinkDetails},
    helpers::{get_ability, get_ability_mut, get_actor, get_actor_mut, get_charge_pool_mut},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct GiveAbilityResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct GiveAbility {
    pub ability_id: ID,
    pub actor_id: ID,
    pub volatile: bool,
}

impl ActionInterface for GiveAbility {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> ActionResult {
        actor.require_system()?;
        get_actor(eng, self.actor_id)?;

        let ability = get_ability(eng, self.ability_id)?;
        let name = ability.ability_name;
        let variant = ability.variant;
        if let Some(owner) = ability.ownership_struct.owner {
            if owner == self.actor_id {
                return Err(ActionError::ItemAlreadyOwned);
            }
            if mutate {
                let other_actor = get_actor_mut(eng, owner).unwrap(); // if
                // the ability is storing the id of an actor that doesn't exist, there is something
                // wrong with the engine.
                other_actor.remove_ability(self.ability_id);
            }
        }

        Action::ClearVolatileLinks(ClearVolatileLinks {
            ability_id: self.ability_id,
        })
        .handle(eng, ctx, actor, version, mutate)?;

        let Some(config) = eng
            .config
            .abilities
            .get(&AbilityIdentifier { name, variant })
        else {
            return Err(ActionError::AbilityConfigNotFound);
        };

        let actor_data = get_actor(eng, self.actor_id)?;
        let conf_links = &config.default_links.clone();
        let mut links_to_create: Vec<PoolLink> = vec![];
        for link in conf_links {
            if let ConfigPoolLinkDetails::Actor(pool_name) = &link.details {
                links_to_create.push(PoolLink {
                    link_type: link.link_type,
                    weight: link.weight,
                    link_dest: *actor_data.pool_map.get(pool_name).unwrap(), // crash on
                                                                             // failure. it must have been created before any abilities.
                });
            }
        }

        let ability = get_ability_mut(eng, self.ability_id)?;
        if mutate {
            ability
                .ownership_struct
                .set_owner(self.actor_id, self.volatile);

            for link in &links_to_create {
                ability.add_link(link.link_dest, link.link_type, link.weight, true);
            }

            for link in &links_to_create {
                let pool = get_charge_pool_mut(eng, link.link_dest)?;
                pool.on_link();
            }

            let actor_data = get_actor_mut(eng, self.actor_id)?;
            actor_data.add_ability(self.ability_id);
        }

        Ok(ActionResponse::GiveAbility(GiveAbilityResponse {}))
    }
}
