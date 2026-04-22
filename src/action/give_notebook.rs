/*
* SYSTEM ACTION
* Give a player true ownership of a notebook
*/

use crate::{
    ID,
    action::{
        ActionActor, ActionError, ActionInterface, ActionResponse, ResponseData, require_player,
    },
    engine::Engine,
};

#[derive(PartialEq, Eq, Clone)]
pub struct GiveNotebookResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct GiveNotebook {
    notebook_id: ID,
    actor_id: ID,
}

impl ActionInterface for GiveNotebook {
    fn validate(&self, eng: &Engine, actor: &ActionActor) -> Result<(), ActionError> {
        actor.require_system()?;
        require_player(eng, self.actor_id)?;
        if eng.world.get_notebook(self.notebook_id).is_none() {
            return Err(ActionError::NotebookNotFound);
        }
        Ok(())
    }

    fn execute(self, eng: &mut Engine, _: &ActionActor) -> ActionResponse {
        let notebook = eng.world.get_notebook_mut(self.notebook_id).unwrap();
        notebook.set_true_owner(self.actor_id);
        ActionResponse {
            commands: vec![],
            next_actions: vec![],
            data: ResponseData::GiveNotebook(GiveNotebookResponse {}),
        }
    }
}
