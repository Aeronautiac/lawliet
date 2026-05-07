/*
* SYSTEM ACTION
* Add a charge pool to the world
*/

use crate::{
    ID,
    action::{ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult},
    chargepool::ChargePool,
    common::{ChargeCount, IterationCount},
};

#[derive(PartialEq, Eq, Clone)]
pub struct AddChargePoolResponse {
    pub id: ID,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct AddChargePool {
    pub base_charges: ChargeCount,
    pub base_reset_time: IterationCount,
}

impl ActionInterface for AddChargePool {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> ActionResult {
        actor.require_system()?;

        let id = if mutate {
            let pool = ChargePool::new(self.base_charges, self.base_reset_time);
            eng.world.add_charge_pool(pool)
        } else {
            0
        };

        Ok(ActionResponse::AddChargePool(AddChargePoolResponse { id }))
    }
}
