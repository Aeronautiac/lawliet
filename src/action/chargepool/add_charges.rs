/*
* SYSTEM ACTION
* Add charges to a pool
*/

use crate::{
    ID,
    action::{ActionInterface, ActionResponse},
    common::ChargeCount,
    helpers::get_charge_pool_mut,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct AddChargesResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct AddCharges {
    pub id: ID,
    pub charges: ChargeCount,
}

impl ActionInterface for AddCharges {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.require_system()?;

        let pool = get_charge_pool_mut(eng, self.id)?;
        if mutate {
            pool.add_charges(self.charges);
        }

        Ok(ActionResponse::AddCharges(AddChargesResponse {}))
    }
}
