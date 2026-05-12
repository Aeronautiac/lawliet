/*
* SYSTEM ACTION
* Remove a link from an ability
*/

// TODO:
// move reference counting here

use crate::{
    ID,
    action::{
        Action, ActionInterface, ActionResponse,
        chargepool::try_delete_charge_pool::TryDeleteChargePool,
    },
    helpers::{get_ability_mut, get_charge_pool, get_charge_pool_mut},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct RemoveLinkResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct RemoveLink {
    pub ability_id: ID,
    pub pool_id: ID,
}

impl ActionInterface for RemoveLink {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.require_system()?;
        get_charge_pool(eng, self.pool_id)?;

        let ability = get_ability_mut(eng, self.ability_id)?;
        if mutate {
            ability.remove_link(self.pool_id);
            let pool = get_charge_pool_mut(eng, self.pool_id)?;
            if pool.on_unlink() {
                Action::TryDeleteChargePool(TryDeleteChargePool { id: self.pool_id })
                    .handle(eng, ctx, actor, version, mutate)?;
            }
        }

        Ok(ActionResponse::RemoveLink(RemoveLinkResponse {}))
    }
}
