/*
* SYSTEM ACTION
* Go through every book with a specific true owner and set the dormant true owner to that person
*/

use crate::{
    ID,
    action::{ActionInterface, ActionResponse},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SetBooksDormantResponse {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SetBooksDormant {
    pub actor_id: ID,
}

impl ActionInterface for SetBooksDormant {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut super::ActionContext,
        actor: &super::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> super::ActionResult {
        actor.require_system()?;

        if mutate {
            for notebook in eng.world.notebooks.values_mut() {
                if notebook.get_true_owner() == Some(self.actor_id) {
                    notebook.set_dormant();
                }
            }
        }

        Ok(ActionResponse::SetBooksDormant(SetBooksDormantResponse {}))
    }
}
