/*
* SYSTEM ACTION
* Kill a player and handle side effects
*/

use crate::{
    ID,
    action::{
        Action, ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult,
        add_state::state_addition, get_actor, get_actor_mut, give_ability::GiveAbility,
        give_notebook::GiveNotebook, give_passive::GivePassive, require_alive,
        sever_links::SeverLinks,
    },
    actor::{ActorType, state::State},
    command::Command,
    common::Version,
    engine::Engine,
};

#[derive(PartialEq, Eq, Clone)]
pub struct KillResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct Kill {
    pub target_id: ID,
    pub killer_id: Option<ID>,
    pub death_message: Option<String>,
    pub silent: bool,
    pub ignore_links: bool,
}

impl ActionInterface for Kill {
    fn handle(
        &mut self,
        eng: &mut Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: Version,
        mutate: bool,
    ) -> ActionResult {
        actor.require_system()?;
        require_alive(eng, self.target_id)?;

        let target = get_actor(eng, self.target_id)?;
        let ActorType::Player(target_data) = &target.actor_type else {
            unreachable!()
        };
        let true_name = target_data.true_name.clone();
        let role = target_data.role;
        let mut notebook_transferred = false;
        let mut ability_transferred = false;

        let mut next_actions = vec![state_addition(self.target_id, State::Dead)];
        if let Some(killer_id) = self.killer_id {
            let killer = get_actor_mut(eng, killer_id)?;

            if mutate {
                killer.kills.push(self.target_id);
            }

            // notebook transfers
            // transfer any currently held notebook to the person who killed them regardless of if
            // the person holding the notebook actually owned the notebook
            for (id, notebook) in eng.world.notebooks.iter() {
                if let Some(owner) = notebook.owner
                    && owner == self.target_id
                // && !notebook.is_owner_borrowing()
                {
                    notebook_transferred = true;
                    next_actions.push(Action::GiveNotebook(GiveNotebook {
                        notebook_id: *id,
                        actor_id: killer_id,
                    }));
                };
            }

            // ability transfers
            for (id, ability) in eng.world.abilities.iter() {
                if let Some(owner) = ability.ownership_struct.owner
                    && owner == self.target_id
                    && ability.ownership_struct.transferrable
                {
                    ability_transferred = true;
                    next_actions.push(Action::GiveAbility(GiveAbility {
                        ability_id: *id,
                        actor_id: killer_id,
                    }));
                }
            }

            // passive transfers
            for (id, passive) in eng.world.passives.iter() {
                if let Some(owner) = passive.ownership_struct.owner
                    && owner == self.target_id
                    && passive.ownership_struct.transferrable
                {
                    ability_transferred = true;
                    next_actions.push(Action::GivePassive(GivePassive {
                        passive_id: *id,
                        actor_id: killer_id,
                    }));
                }
            }
        }

        if !self.ignore_links {
            let target = get_actor(eng, self.target_id)?;

            // life links
            let links = target.actor_links.clone();
            for link in links {
                next_actions.push(Action::Kill(Kill {
                    target_id: link.link_dest,
                    silent: self.silent,
                    death_message: Some(eng.config.defaults.life_link_death_message.clone()),
                    ignore_links: false,
                    killer_id: self.killer_id,
                }));
            }

            // sever all links pointing to this actor
            next_actions.push(Action::SeverLinks(SeverLinks {
                actor_id: self.target_id,
            }));
        }

        for mut action in next_actions {
            action.handle(eng, ctx, actor, version, mutate)?;
        }

        if !self.silent {
            ctx.commands.push(Command::AnnounceDeath {
                true_name: String::from(&*true_name),
                death_message: if let Some(msg) = &self.death_message {
                    msg.clone()
                } else {
                    eng.config.defaults.death_message.clone()
                },
                role,
                notebook_transferred,
                ability_transferred,
            });
        }

        Ok(ActionResponse::Kill(KillResponse {}))
    }
}
