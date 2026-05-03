/*
* SYSTEM ACTION
* Take a notebook away from someone (set the owner to None)
*/

use crate::{
    ID,
    action::{ActionError, ActionInterface, ActionResponse},
    helpers::{get_actor_mut, get_notebook, get_notebook_mut},
};

#[derive(PartialEq, Eq, Clone)]
pub struct TakeNotebookResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct TakeNotebook {
    pub notebook_id: ID,
}

impl ActionInterface for TakeNotebook {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut super::ActionContext,
        actor: &super::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> super::ActionResult {
        actor.require_system()?;

        let notebook = get_notebook(eng, self.notebook_id)?;
        if notebook.get_true_owner().is_none() {
            return Err(ActionError::ItemAlreadyUnowned);
        }
        if mutate {
            if let Some(owner) = notebook.owner {
                let owner_actor = get_actor_mut(eng, owner).unwrap();
                owner_actor.remove_notebook(self.notebook_id);
            }
            let notebook = get_notebook_mut(eng, self.notebook_id)?;
            notebook.strip_ownership();
        }

        Ok(ActionResponse::TakeNotebook(TakeNotebookResponse {}))
    }
}
