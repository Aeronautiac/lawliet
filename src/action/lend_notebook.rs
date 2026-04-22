/*
* PLAYER ACTION
* Lend a notebook to another player
*/

use crate::{
    ID,
    action::{ActionActor, ActionError, ActionInterface, ActionResponse, ResponseData, actor_id},
    actor::restriction::Restriction,
    engine::Engine,
};

#[derive(Debug)]
pub struct LendNotebookResponse {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LendNotebook {
    notebook_id: ID,
    target_id: ID,
}

impl ActionInterface for LendNotebook {
    fn validate(&self, eng: &Engine, actor: &ActionActor) -> Result<(), ActionError> {
        actor.player_only()?;
        let player_id = actor_id(actor).unwrap();

        let Some(book) = eng.world.get_notebook(self.notebook_id) else {
            return Err(ActionError::NotebookNotFound);
        };

        let player_actor = eng.world.get_actor(player_id).unwrap();
        if player_actor.has_restriction(Restriction::NotebookPassage) {
            return Err(ActionError::NotebookPassageBlocked);
        }

        if book.can_lend(player_id).is_err() {
            return Err(ActionError::NotebookNotOwned);
        }

        Ok(())
    }

    fn execute(self, eng: &mut Engine, _: &ActionActor) -> ActionResponse {
        let book = eng.world.get_notebook_mut(self.notebook_id).unwrap();
        book.lend(self.target_id).unwrap();

        ActionResponse {
            commands: vec![],
            next_actions: vec![],
            data: ResponseData::LendNotebook(LendNotebookResponse {}),
        }
    }
}
