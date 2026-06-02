/*
* SYSTEM ACTION
* Add a charge pool to the world
*/

use crate::{
    action::{ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult},
    chargepool::PoolLinkType,
    common::{AbilityKey, ChargePoolKey, LinkWeight},
    helpers::{get_ability_mut, get_charge_pool, get_charge_pool_mut},
};

#[derive(PartialEq, Eq, Clone)]
pub struct AddLinkResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct AddLink {
    pub ability_id: AbilityKey,
    pub pool_id: ChargePoolKey,
    pub weight: LinkWeight,
    pub link_type: PoolLinkType,
    pub volatile: bool,
}

impl ActionInterface for AddLink {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> ActionResult {
        actor.admin_or_system()?;
        get_charge_pool(eng, self.pool_id)?;

        let ability = get_ability_mut(eng, self.ability_id)?;
        if mutate {
            ability.add_link(self.pool_id, self.link_type, self.weight, self.volatile);
            let pool = get_charge_pool_mut(eng, self.pool_id)?;
            pool.on_link();
        }

        Ok(ActionResponse::AddLink(AddLinkResponse {}))
    }
}
