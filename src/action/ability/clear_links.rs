/*
* SYSTEM ACTION
* Remove all links from an ability
*/

use crate::{
    ID,
    action::{Action, ActionInterface, ActionResponse, ability::remove_link::RemoveLink},
    helpers::{get_ability_mut, get_charge_pool},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ClearLinksResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ClearLinks {
    pub ability_id: ID,
}

impl ActionInterface for ClearLinks {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.require_system()?;

        let ability = get_ability_mut(eng, self.ability_id)?;
        let links = ability.pool_links.clone();
        for container in links {
            Action::RemoveLink(RemoveLink {
                ability_id: self.ability_id,
                pool_id: container.link.link_dest,
            })
            .handle(eng, ctx, actor, version, mutate)?;
        }

        Ok(ActionResponse::ClearLinks(ClearLinksResponse {}))
    }
}
