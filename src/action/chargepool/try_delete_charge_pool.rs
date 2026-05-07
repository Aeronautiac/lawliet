/*
* SYSTEM ACTION
* Try to delete a charge pool (check the reference count)
*/

use crate::{
    ID,
    action::{ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult},
    helpers::get_charge_pool,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct TryDeleteChargePoolResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct TryDeleteChargePool {
    pub id: ID,
}

impl ActionInterface for TryDeleteChargePool {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> ActionResult {
        actor.require_system()?;
        let pool = get_charge_pool(eng, self.id)?;

        if mutate && pool.ref_count == 0 {
            eng.world.remove_charge_pool(self.id);
        }

        Ok(ActionResponse::TryDeleteChargePool(
            TryDeleteChargePoolResponse {},
        ))
    }
}
