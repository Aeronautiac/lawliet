/*
* SYSTEM ACTION
* Kill a player and handle side effects
*/

use crate::{
    ID,
    action::{
        Action, ActionActor, ActionInterface, ActionResponse, ActionResult, Command, ResponseData,
        add_state::state_addition, get_actor, get_actor_mut, give_notebook::GiveNotebook,
        require_alive,
    },
    actor::{ActorType, state::State},
    common::Version,
    config::role::Role,
    engine::Engine,
};

#[derive(PartialEq, Eq, Clone)]
pub struct KillResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct Kill {
    pub target_id: ID,
    pub killer_id: Option<ID>,
    pub death_message: Option<String>,
}

impl ActionInterface for Kill {
    fn handle(
        &mut self,
        eng: &mut Engine,
        actor: &ActionActor,
        _: Version,
        mutate: bool,
    ) -> ActionResult {
        actor.require_system()?;
        require_alive(eng, self.target_id)?;

        let target = get_actor(eng, self.target_id)?;
        let ActorType::Player(target_data) = &target.actor_type else {
            unreachable!()
        };
        let true_name = target_data.true_name.clone();

        let mut next_actions = vec![state_addition(self.target_id, State::Dead)];
        if let Some(killer_id) = self.killer_id {
            let killer = get_actor_mut(eng, killer_id)?;

            if mutate {
                killer.kills.push(self.target_id);
            }

            // notebook transfers
            for (id, notebook) in eng.world.notebooks.iter() {
                if let Some(owner) = notebook.owner
                    && owner == self.target_id
                    && !notebook.is_owner_borrowing()
                {
                    next_actions.push(Action::GiveNotebook(GiveNotebook {
                        notebook_id: *id,
                        actor_id: killer_id,
                    }));
                };
            }
        }

        Ok(ActionResponse {
            commands: vec![Command::AnnounceDeath {
                true_name: String::from(&*true_name),
                death_message: if let Some(msg) = &self.death_message {
                    msg.clone()
                } else {
                    String::from("Placeholder default")
                },
                role: Role::Civilian,
                had_notebook: false,
            }],
            next_actions,
            data: ResponseData::Kill(KillResponse {}),
        })
    }
}
