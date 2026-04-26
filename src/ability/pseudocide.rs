use crate::{
    ability::{AbilityInterface, AbilityResponseData},
    action::{actor_id, require_alive},
    config::ability::AbilityName,
};

pub struct PseudocideResponse {}

pub struct Pseudocide {}

impl AbilityInterface for Pseudocide {
    fn ability_name(&self) -> crate::config::ability::AbilityName {
        AbilityName::Pseudocide
    }

    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        actor: &crate::action::ActionActor,
        ability: &mut super::Ability,
        version: u8,
        mutate: bool,
    ) -> super::AbilityResult {
        Ok(super::AbilityResponse {
            actions: vec![],
            data: AbilityResponseData::Psuedocide(PseudocideResponse {}),
        })
    }
}
