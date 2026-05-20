/*
* SYSTEM ACTION
* Create the world's base organizations
*/

use crate::action::{ActionInterface, ActionResponse};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CreateOrgsResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CreateOrgs {}

impl ActionInterface for CreateOrgs {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.require_system()?;

        // TODO:
        // implement it

        Ok(ActionResponse::CreateOrgs(CreateOrgsResponse {}))
    }
}

