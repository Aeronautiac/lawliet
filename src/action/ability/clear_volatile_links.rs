/*
* SYSTEM ACTION
* Clear all of an ability's unowned links. Decrement charge pool reference count and try to destroy
* it if ref count hits zero.
*/

use crate::{
    ID,
    action::{
        Action, ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult,
        ability::remove_link::RemoveLink, chargepool::try_delete_charge_pool::TryDeleteChargePool,
    },
    helpers::{get_ability_mut, get_charge_pool_mut},
};

#[derive(PartialEq, Eq, Clone)]
pub struct ClearVolatileLinksResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ClearVolatileLinks {
    pub ability_id: ID,
}

impl ActionInterface for ClearVolatileLinks {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> ActionResult {
        actor.require_system()?;
        let ability = get_ability_mut(eng, self.ability_id)?;

        let mut links_to_destroy = vec![];
        for link in &ability.pool_links {
            if link.volatile {
                links_to_destroy.push(link.link.link_dest);
            }
        }

        for id in &links_to_destroy {
            Action::RemoveLink(RemoveLink {
                ability_id: self.ability_id,
                pool_id: *id,
            })
            .handle(eng, ctx, actor, version, mutate)?;
        }

        Ok(ActionResponse::ClearVolatileLinks(
            ClearVolatileLinksResponse {},
        ))
    }
}
