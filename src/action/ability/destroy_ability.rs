/*
* SYSTEM ACTION
* Fully destroy an ability: clear all pool links, remove from the owning actor's cache,
* drop any bugs referencing this ability, then remove from the world.
*/

use crate::{
    ID,
    action::{
        Action, ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult,
        ability::remove_link::RemoveLink,
        comms::bug::destroy_bug::DestroyBug,
    },
    helpers::{get_ability, get_actor, get_actor_mut},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DestroyAbilityResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DestroyAbility {
    pub ability_id: ID,
}

impl ActionInterface for DestroyAbility {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> ActionResult {
        actor.require_system()?;

        let ability = get_ability(eng, self.ability_id)?;
        let owner = ability.ownership_struct.owner;
        let pool_ids: Vec<ID> = ability.pool_links.iter().map(|l| l.link.link_dest).collect();

        if let Some(owner_id) = owner {
            get_actor(eng, owner_id)?;
        }

        for pool_id in pool_ids {
            Action::RemoveLink(RemoveLink {
                ability_id: self.ability_id,
                pool_id,
            })
            .handle(eng, ctx, actor, version, mutate)?;
        }

        let bug_ids: Vec<ID> = eng
            .world
            .bugs
            .iter()
            .filter(|(_, bug)| bug.ability_id == self.ability_id)
            .map(|(id, _)| *id)
            .collect();

        for bug_id in bug_ids {
            Action::DestroyBug(DestroyBug { bug_id }).handle(eng, ctx, actor, version, mutate)?;
        }

        if mutate {
            if let Some(owner_id) = owner {
                get_actor_mut(eng, owner_id)
                    .expect("ability owner does not exist: engine invariant violated")
                    .remove_ability(self.ability_id);
            }
            eng.world.remove_ability(self.ability_id);
        }

        Ok(ActionResponse::DestroyAbility(DestroyAbilityResponse {}))
    }
}
