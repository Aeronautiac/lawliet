/*
* SYSTEM ACTION
* Add a notebook to the world state
*/

use crate::{
    ID,
    action::{ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult},
    common::Version,
    engine::Engine,
};

#[derive(PartialEq, Eq, Clone)]
pub struct AddNotebookResponse {
    pub id: ID, // return the internal id assigned to this notebook
}

#[derive(PartialEq, Eq, Clone)]
pub struct AddNotebook {
    pub fake: bool,
}

impl ActionInterface for AddNotebook {
    fn handle(
        &mut self,
        eng: &mut Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        _: Version,
        mutate: bool,
    ) -> ActionResult {
        actor.require_system()?;

        let id = if mutate {
            eng.world.add_notebook(self.fake)
        } else {
            0
        };

        Ok(ActionResponse::AddNotebook(AddNotebookResponse { id }))
    }
}
