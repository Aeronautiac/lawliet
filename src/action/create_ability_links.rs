/*
* SYSTEM ACTION
* Go through every ability owned by a certain actor and apply any links that config dictates
* Apply links through the ability API directly. This action has no sub-actions.
*/

use crate::{
    ID,
    action::{
        ActionContext, ActionInterface, ActionResponse, get_ability, get_ability_config,
        get_ability_mut, get_actor,
    },
};

#[derive(PartialEq, Eq, Clone)]
pub struct CreateAbilityLinksResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct CreateAbilityLinks {
    pub target_id: ID,
}

impl ActionInterface for CreateAbilityLinks {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut ActionContext,
        actor: &super::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> super::ActionResult {
        actor.require_system()?;
        get_actor(eng, self.target_id)?;

        let owned_abilities: Vec<ID> = eng
            .world
            .abilities
            .iter()
            .filter(|(_id, ability)| ability.ownership_struct.owner == Some(self.target_id))
            .map(|(id, _ability)| *id)
            .collect();

        for ability_id in &owned_abilities {
            let config = get_ability_config(eng, *ability_id)?;
            let links = config.default_links.clone();
            for link in links {
                for other_ability_id in &owned_abilities {
                    let other_ability = get_ability(eng, *other_ability_id)?;
                    let variant = other_ability.variant;
                    let name = other_ability.ability_name;
                    let ability = get_ability_mut(eng, *ability_id)?;
                    if mutate && link.identifier.variant == variant && link.identifier.name == name
                    {
                        ability.add_link(*other_ability_id, link.link_type, link.weight);
                    }
                }
            }
        }

        Ok(ActionResponse::CreateAbilityLinks(
            CreateAbilityLinksResponse {},
        ))
    }
}
