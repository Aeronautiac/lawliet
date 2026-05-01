/*
* SYSTEM ACTION
* Give a player true ownership of a notebook
*/

use crate::{
    ID,
    action::{
        ActionActor, ActionContext, ActionError, ActionInterface, ActionResponse, ActionResult,
        get_actor_mut, get_notebook, get_notebook_mut, require_player,
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
    pub volatile: bool,
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

        let notebook = get_notebook(eng, self.notebook_id)?;
        if let Some(curr_owner) = notebook.owner {
            if curr_owner == self.actor_id {
                return Err(ActionError::ItemAlreadyOwned);
            }
            if mutate {
                let other_actor = get_actor_mut(eng, curr_owner).unwrap(); // if
                // the owner doesn't exist, there's something wrong with the engine
                other_actor.remove_notebook(self.notebook_id);
            }
        }

        let notebook = get_notebook_mut(eng, self.notebook_id)?;
        if mutate {
            notebook.set_true_owner(self.actor_id, self.volatile);
            let actor = get_actor_mut(eng, self.actor_id)?;
            actor.add_notebook(self.notebook_id);
        }

        Ok(ActionResponse::GiveNotebook(GiveNotebookResponse {}))
    }
}
