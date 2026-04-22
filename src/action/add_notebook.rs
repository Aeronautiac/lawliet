/*
* SYSTEM ACTION
* Add a notebook to the world state
*/

use crate::{
    ID,
    action::{ActionActor, ActionError, ActionInterface, ActionResponse, ResponseData},
    engine::Engine,
};

#[derive(PartialEq, Eq, Clone)]
pub struct AddNotebookResponse {
    pub id: ID, // return the internal id assigned to this notebook
}

#[derive(PartialEq, Eq, Clone)]
pub struct AddNotebook {
    fake: bool,
}

impl ActionInterface for AddNotebook {
    fn validate(&self, _: &Engine, actor: &ActionActor) -> Result<(), ActionError> {
        actor.require_system()?;
        Ok(())
    }

    fn execute(self, eng: &mut Engine, _: &ActionActor) -> ActionResponse {
        let id = eng.world.add_notebook(self.fake);
        ActionResponse {
            commands: vec![],
            next_actions: vec![],
            data: ResponseData::AddNotebook(AddNotebookResponse { id }),
        }
    }
}
