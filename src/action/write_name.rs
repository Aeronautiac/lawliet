/*
* PLAYER ACTION
* Write a player's name in a notebook
* IPP blocks this
*/

use crate::{
    ID, Timestamp,
    action::{
        Action, ActionActor, ActionContext, ActionError, ActionInterface, ActionResponse,
        ActionResult, actor_id, kill::Kill, schedule_kill::ScheduleKill,
    },
    actor::restriction::Restriction,
    common::Version,
    engine::Engine,
    notebook::NotebookError,
};

#[derive(Debug)]
pub struct WriteNameResponse {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WriteName {
    pub true_name: String,
    pub death_message: Option<String>,
    pub notebook_id: ID,
    pub delay: Option<Timestamp>, // the time (in seconds) after the current time to kill the player
}

impl ActionInterface for WriteName {
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
        let target = eng.world.get_player_id_by_name(&self.true_name);

        let player_actor = eng.world.get_actor_mut(player_id).unwrap();
        if player_actor.has_restriction(Restriction::NotebookUsage) {
            return Err(ActionError::NotebookUsageBlocked);
        }

        let Some(book) = eng.world.get_notebook_mut(self.notebook_id) else {
            return Err(ActionError::NotebookNotFound);
        };

        // still need to implement game rule blockage

        // temporarily hardcoded as config is not fully implemented yet
        if let Err(error) = book.can_write(player_id, 3, 1) {
            return Err(match error {
                NotebookError::NoOwner | NotebookError::NotOwned => ActionError::NotebookNotOwned,
                NotebookError::OnCooldown => ActionError::NotebookOnCooldown,
            });
        }

        if let Some(target_id) = target
            && !book.fake
        {
            if mutate {
                book.on_write_success(player_id);
            }
            if let Some(delay) = self.delay {
                Action::ScheduleKill(ScheduleKill {
                    timestamp: eng.time + delay,
                    kill: Kill {
                        target_id,
                        killer_id: Some(player_id),
                        death_message: self.death_message.clone(),
                        silent: false,
                    },
                })
                .handle(eng, ctx, actor, version, mutate)?;
            } else {
                Action::Kill(Kill {
                    target_id,
                    killer_id: Some(player_id),
                    death_message: self.death_message.clone(),
                    silent: false,
                })
                .handle(eng, ctx, actor, version, mutate)?;
            }
            Ok(ActionResponse::WriteName(WriteNameResponse {}))
        } else {
            if mutate {
                book.on_write_failure(player_id);
            }
            Ok(ActionResponse::WriteName(WriteNameResponse {}))
        }
    }
}
