/*
* SYSTEM ACTION
* Transfer ownership of an ability to a specified actor and then reset links
*/

use crate::{
    ID,
    action::{
        Action, ActionContext, ActionInterface, ActionResponse,
        create_ability_links::CreateAbilityLinks, get_ability_mut, get_actor, get_actor_mut,
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
        ctx: &mut ActionContext,
        actor: &super::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> super::ActionResult {
        actor.require_system()?;
        get_actor(eng, self.actor_id)?;

        let ability = get_ability_mut(eng, self.ability_id)?;
        if mutate {
            ability.clear_links();
            ability.ownership_struct.set_owner(self.actor_id);
            let actor_data = get_actor_mut(eng, self.actor_id)?;
            actor_data.add_ability(self.ability_id);
        }

        Action::CreateAbilityLinks(CreateAbilityLinks {
            target_id: self.actor_id,
        })
        .handle(eng, ctx, actor, version, mutate)?;

        Ok(ActionResponse::GiveAbility(GiveAbilityResponse {}))
    }
}
