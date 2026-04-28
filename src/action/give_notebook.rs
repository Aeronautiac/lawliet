/*
* SYSTEM ACTION
* Give a player true ownership of a notebook
*/

use crate::{
    ID,
    action::{
        ActionActor, ActionContext, ActionError, ActionInterface, ActionResponse, ActionResult,
        require_player,
    },
    common::Version,
    engine::Engine,
};

#[derive(PartialEq, Eq, Clone)]
pub struct GiveNotebookResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct GiveNotebook {
    pub notebook_id: ID,
    pub actor_id: ID,
}

impl ActionInterface for GiveNotebook {
    fn handle(
        &mut self,
        eng: &mut Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        _: Version,
        mutate: bool,
    ) -> ActionResult {
        actor.require_system()?;
        require_player(eng, self.actor_id)?;

        let notebook = eng.world.get_notebook_mut(self.notebook_id);
        if notebook.is_none() {
            return Err(ActionError::NotebookNotFound);
        }
        let notebook = notebook.unwrap();

        if mutate {
            notebook.set_true_owner(self.actor_id);
        }

        Ok(ActionResponse::GiveNotebook(GiveNotebookResponse {}))
    }
}
