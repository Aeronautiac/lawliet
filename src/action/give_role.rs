/*
* SYSTEM ACTION
* Change a player's role and grant them abilities associated with that role
* If a player already has the requested role, return an error
* Changing a player's role deletes any of their old "volatile" abilities
* A volatile ability is one which disappears on role change
*/

use crate::{
    ID,
    action::{ActionContext, ActionInterface},
    config::role::Role,
};

pub struct GiveRoleResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct GiveRole {
    pub target_id: ID,
    pub role: Role,
}

impl ActionInterface for GiveRole {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut ActionContext,
        actor: &super::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> super::ActionResult {
        actor.require_system()?;
        unimplemented!()
    }
}
