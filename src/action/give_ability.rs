/*
* SYSTEM ACTION
* Transfer ownership of an ability to a specified actor and then reset links
*/

use crate::{
    ID,
    action::{
        Action, ActionInterface, ActionResponse, ResponseData,
        create_ability_links::CreateAbilityLinks, get_ability_mut, get_actor,
    },
};

#[derive(PartialEq, Eq, Clone)]
pub struct GiveAbilityResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct GiveAbility {
    pub ability_id: ID,
    pub actor_id: ID,
}

impl ActionInterface for GiveAbility {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        actor: &super::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> super::ActionResult {
        actor.require_system()?;
        get_actor(eng, self.actor_id)?;

        let ability = get_ability_mut(eng, self.ability_id)?;
        if mutate {
            ability.clear_links();
            ability.set_owner(self.actor_id);
        }

        Ok(ActionResponse {
            commands: vec![],
            next_actions: vec![Action::CreateAbilityLinks(CreateAbilityLinks {
                target_id: self.actor_id,
            })],
            data: ResponseData::GiveAbility(GiveAbilityResponse {}),
        })
    }
}
