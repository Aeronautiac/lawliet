/*
* SYSTEM ACTION
* Return all notebooks with dormant true owner equal to actor_id to the dormant true owner
*/

use crate::{
    ID,
    action::{ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ReturnDormantBooksResponse {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ReturnDormantBooks {
    pub actor_id: ID,
}

impl ActionInterface for ReturnDormantBooks {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> ActionResult {
        actor.require_system()?;

        if mutate {
            for notebook in eng.world.notebooks.values_mut() {
                if notebook.get_dormant_owner() == Some(self.actor_id) {
                    notebook.awaken_dormant_owner();
                }
            }
        }

        Ok(ActionResponse::ReturnDormantBooks(
            ReturnDormantBooksResponse {},
        ))
    }
}
