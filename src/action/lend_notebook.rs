/*
* PLAYER ACTION
* Lend a notebook to another player
*/

use crate::{
    ID,
    action::{
        ActionActor, ActionContext, ActionError, ActionInterface, ActionResponse, ActionResult,
    },
    actor::restriction::Restriction,
    common::Version,
    engine::Engine,
    helpers::{actor_id, get_actor, get_actor_mut, get_notebook, get_notebook_mut, require_alive},
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

        let user_id = actor_id(actor).unwrap();
        if user_id == self.target_id {
            return Err(ActionError::CannotLendToYourself);
        }

        let notebook = get_notebook_mut(eng, self.notebook_id)?;
        if notebook.can_lend(user_id).is_err() {
            return Err(ActionError::NotebookNotOwned);
        }
        if mutate {
            notebook.lend(self.target_id).unwrap();
        }

        let player_actor = get_actor_mut(eng, user_id)?;
        if player_actor.has_restriction(Restriction::NotebookPassage) {
            return Err(ActionError::NotebookPassageBlocked);
        }
        if mutate {
            player_actor.remove_notebook(self.notebook_id);
        }

        let target_actor = get_actor_mut(eng, self.target_id)?;
        if target_actor.has_restriction(Restriction::NotebookReceive) {
            return Err(ActionError::ActorHasNotebookReceiveRestriction);
        }
        if mutate {
            target_actor.add_notebook(self.notebook_id);
        }

        Ok(ActionResponse::LendNotebook(LendNotebookResponse {}))
    }
}
