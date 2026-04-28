/*
* SYSTEM ACTION
* Transfer ownership of an ability to a specified actor and then reset links
*/

use crate::{
    ID,
    action::{
        Action, ActionContext, ActionInterface, ActionResponse,
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
            ability.set_owner(self.actor_id);
        }

        // potential issue with the version system here
        // if i change to the latest version for this specific ability, and then i pass it down to
        // the next, but that ability is still at version 0, then what do i do? i need to explicitly
        // pick sub-action versions depending on version here. not a big problem.
        Action::CreateAbilityLinks(CreateAbilityLinks {
            target_id: self.actor_id,
        })
        .handle(eng, ctx, actor, 0, mutate)?;

        Ok(ActionResponse::GiveAbility(GiveAbilityResponse {}))
    }
}
