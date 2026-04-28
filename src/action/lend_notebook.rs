/*
* PLAYER ACTION
* Lend a notebook to another player
*/

use crate::{
    ID,
    action::{
        ActionActor, ActionContext, ActionError, ActionInterface, ActionResponse, ActionResult,
        actor_id,
    },
    actor::restriction::Restriction,
    common::Version,
    engine::Engine,
};

#[derive(Debug)]
pub struct LendNotebookResponse {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LendNotebook {
    pub notebook_id: ID,
    pub target_id: ID,
}

impl ActionInterface for LendNotebook {
    fn handle(
        &mut self,
        eng: &mut Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: Version,
        mutate: bool,
    ) -> ActionResult {
        actor.player_only()?;
        let player_id = actor_id(actor).unwrap();

        let player_actor = eng.world.get_actor(player_id).unwrap();
        if player_actor.has_restriction(Restriction::NotebookPassage) {
            return Err(ActionError::NotebookPassageBlocked);
        }

        let Some(book) = eng.world.get_notebook_mut(self.notebook_id) else {
            return Err(ActionError::NotebookNotFound);
        };

        if book.can_lend(player_id).is_err() {
            return Err(ActionError::NotebookNotOwned);
        }

        if mutate {
            book.lend(self.target_id).unwrap();
        }

        Ok(ActionResponse::LendNotebook(LendNotebookResponse {}))
    }
}
