/*
* SYSTEM ACTION
* Go through every ability owned by a certain actor and apply any links that config dictates
* Apply links through the ability API directly. This action has no sub-actions.
*/

use crate::{
    ID,
    action::{ActionInterface, ActionResponse, ResponseData},
};

#[derive(PartialEq, Eq, Clone)]
pub struct CreateAbilityLinksResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct CreateAbilityLinks {
    target_id: ID,
}

impl ActionInterface for CreateAbilityLinks {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        actor: &super::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> super::ActionResult {
        actor.require_system()?;

        let owned_abilities: Vec<ID> = eng
            .world
            .abilities
            .keys()
            .cloned()
            .filter(|id| *id == self.target_id)
            .collect();

        if mutate {
            for ability in &owned_abilities {
                // TODO: link the ability to every other ability it must be linked to based on config
                for other_ability in &owned_abilities {}
            }
        }

        Ok(ActionResponse {
            next_actions: vec![],
            commands: vec![],
            data: ResponseData::CreateAbilityLinks(CreateAbilityLinksResponse {}),
        })
    }
}
