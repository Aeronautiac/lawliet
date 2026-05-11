/*
* PLAYER ACTION
* Write a player's name in a notebook
* IPP blocks this
*/

use crate::{
    ID, Time,
    action::{
        Action, ActionActor, ActionContext, ActionError, ActionInterface, ActionResponse,
        ActionResult,
        actor::player::{kill::Kill, schedule_kill::ScheduleKill},
    },
    actor::modifier::Modifier,
    common::Version,
    engine::Engine,
    helpers::actor_id,
    notebook::NotebookError,
};

#[derive(Debug)]
pub struct WriteNameResponse {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WriteName {
    pub true_name: String,
    pub death_message: Option<String>,
    pub notebook_id: ID,
    pub delay: Time, // the time (in seconds) after the current time to kill the player
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
        if player_actor.has_modifier(Modifier::NoNotebookUsage) {
            return Err(ActionError::NotebookUsageBlocked);
        }

        let Some(book) = eng.world.get_notebook_mut(self.notebook_id) else {
            return Err(ActionError::NotebookNotFound);
        };

        // need to implement blockage by the notebook restrict passive

        // temporarily hardcoded as config is not fully implemented yet
        if let Err(error) = book.can_write(
            player_id,
            eng.config.defaults.notebook_failures_per_day,
            eng.config.defaults.notebook_successes_per_day,
        ) {
            return Err(match error {
                NotebookError::NoOwner | NotebookError::NotOwned => ActionError::NotebookNotOwned,
                NotebookError::OnCooldown => ActionError::NotebookOnCooldown,
            });
        }

        if let Some(target_id) = target
            && !book.fake
        {
            let mut cancelled = false;
            for job in eng.jobs.iter() {
                if *job.cancelled.borrow_mut() {
                    continue;
                }
                if let Action::NotebookScheduledKill(data) = &job.request.payload
                    && data.kill.target_id == target_id
                {
                    cancelled = true;
                    if mutate {
                        *job.cancelled.borrow_mut() = true;
                    }
                }
            }
            if mutate {
                book.on_write_success(player_id);
            }
            if !cancelled {
                if self.delay > 0 {
                    Action::ScheduleKill(ScheduleKill {
                        timestamp: eng.time + self.delay,
                        notebook_scheduled: true,
                        kill: Kill {
                            allow_link_chaining: true,
                            sever_links: true,
                            set_books_dormant: false,
                            target_id,
                            killer_id: Some(player_id),
                            death_message: self.death_message.clone(),
                            silent: false,
                        },
                    })
                    .handle(
                        eng,
                        ctx,
                        &ActionActor::System,
                        version,
                        mutate,
                    )?;
                } else {
                    Action::Kill(Kill {
                        allow_link_chaining: true,
                        sever_links: true,
                        set_books_dormant: false,
                        target_id,
                        killer_id: Some(player_id),
                        death_message: self.death_message.clone(),
                        silent: false,
                    })
                    .handle(
                        eng,
                        ctx,
                        &ActionActor::System,
                        version,
                        mutate,
                    )?;
                }
                Ok(ActionResponse::WriteName(WriteNameResponse {}))
            } else {
                Ok(ActionResponse::WriteName(WriteNameResponse {}))
            }
        } else {
            if mutate {
                book.on_write_failure(player_id);
            }
            Ok(ActionResponse::WriteName(WriteNameResponse {}))
        }
    }
}
