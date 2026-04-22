/*
* PLAYER ACTION
* Write a player's name in a notebook
*/

use crate::{
    ID,
    action::{
        Action, ActionActor, ActionError, ActionInterface, ActionResponse, ResponseData, actor_id,
        kill::Kill, require_player,
    },
    actor::restriction::Restriction,
    engine::Engine,
    notebook::{Notebook, NotebookError},
};

#[derive(Debug)]
pub struct WriteNameResponse {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WriteName {
    true_name: String,
    death_message: Option<String>,
    notebook_id: ID,
}

impl ActionInterface for WriteName {
    // the notebook must exist
    // the player must own the notebook
    // the player must not be blocked by a game rule
    // the player must not have a notebook writing restriction
    // the number of successes must be < the iteration limit
    // the number of failures must be < the iteration limit
    fn validate(&self, eng: &Engine, actor: &ActionActor) -> Result<(), ActionError> {
        actor.player_only()?;
        let player_id = actor_id(actor).unwrap();

        let Some(book) = eng.world.get_notebook(self.notebook_id) else {
            return Err(ActionError::NotebookNotFound);
        };
        let player_actor = eng.world.get_actor(player_id).unwrap();
        if player_actor.has_restriction(Restriction::NotebookUsage) {
            return Err(ActionError::NotebookUsageBlocked);
        }

        // still need to implement game rule blockage

        // temporarily hardcoded as config is not fully implemented yet
        if let Err(error) = book.can_write(player_id, 3, 1) {
            return Err(match error {
                NotebookError::NoOwner | NotebookError::NotOwned => ActionError::NotebookNotOwned,
                NotebookError::OnCooldown => ActionError::NotebookOnCooldown,
            });
        }

        Ok(())
    }

    fn execute(self, eng: &mut Engine, actor: &ActionActor) -> ActionResponse {
        let player_id = actor_id(actor).unwrap();
        let target = eng.world.get_player_id_by_name(&self.true_name);
        let book = eng.world.get_notebook_mut(self.notebook_id).unwrap();

        if let Some(target_id) = target {
            book.on_write_success(player_id);

            ActionResponse {
                commands: vec![], // later tell the frontend to acknowledge the kill or similar. not
                // important right now as the focus is on working game logic.
                next_actions: vec![Action::Kill(Kill {
                    target_id,
                    killer_id: Some(player_id),
                    death_message: self.death_message,
                })],
                data: ResponseData::WriteName(WriteNameResponse {}),
            }
        } else {
            book.on_write_failure(player_id);

            ActionResponse {
                commands: vec![], // tell the frontend to inform the player of the write failure
                // along with the number of attempts remaining
                next_actions: vec![],
                data: ResponseData::WriteName(WriteNameResponse {}),
            }
        }
    }
}
