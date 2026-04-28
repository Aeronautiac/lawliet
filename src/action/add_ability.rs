/*
* SYSTEM ACTION
* Add an ability to the world
*/

use crate::{
    ID,
    ability::Ability,
    action::{ActionError, ActionInterface, ActionResponse, ResponseData},
    common::Variant,
    config::ability::{AbilityIdentifier, AbilityName},
};

#[derive(PartialEq, Eq, Clone)]
pub struct AddAbilityResponse {
    id: ID,
}

#[derive(PartialEq, Eq, Clone)]
pub struct AddAbility {
    pub ability_name: AbilityName,
    pub volatile: bool,
    pub variant: Variant,
}

impl ActionInterface for AddAbility {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        actor: &super::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> super::ActionResult {
        actor.require_system()?;

        let Some(config) = eng.config.abilities.get(&AbilityIdentifier {
            name: self.ability_name,
            variant: self.variant,
        }) else {
            return Err(ActionError::AbilityConfigNotFound);
        };

        let id = if mutate {
            let ability = Ability::new(
                self.ability_name,
                self.variant,
                config.base_charges,
                self.volatile,
            );
            eng.world.add_ability(ability)
        } else {
            0
        };

        Ok(ActionResponse {
            commands: vec![],
            next_actions: vec![],
            data: ResponseData::AddAbility(AddAbilityResponse { id }),
        })
    }
}
