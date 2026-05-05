/*
* SYSTEM ACTION
* Initialize any necessary world state
*/

use crate::{
    action::{Action, ActionInterface, ActionResponse, add_charge_pool::AddChargePool},
    helpers::get_charge_pool_mut,
};

pub struct InitializeWorldResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct InitializeWorld {}

impl ActionInterface for InitializeWorld {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut super::ActionContext,
        actor: &super::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> super::ActionResult {
        actor.require_system()?;

        let pool_config = eng.config.world_config.charge_pools.clone();
        for (name, specifier) in pool_config {
            let response = Action::AddChargePool(AddChargePool {
                base_charges: specifier.charges,
                base_reset_time: specifier.reset_time,
            })
            .handle(eng, ctx, actor, version, mutate)?;
            if mutate {
                let ActionResponse::AddChargePool(data) = response else {
                    unreachable!()
                };
                let pool = get_charge_pool_mut(eng, data.id)?;
                pool.on_link();
                eng.world.pool_map.insert(name, data.id);
            }
        }

        Ok(ActionResponse::InitializeWorld(InitializeWorldResponse {}))
    }
}
