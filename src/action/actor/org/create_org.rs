/*
* SYSTEM ACTION
* Create a new organization
* Use config to determine details
* Remember than an org is just a variant of an actor
*/

use crate::action::{ActionInterface, ActionResponse};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CreateOrgResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CreateOrg {}

impl ActionInterface for CreateOrg {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.require_system()?;

        Ok(ActionResponse::CreateOrg(CreateOrgResponse {}))
    }
}
