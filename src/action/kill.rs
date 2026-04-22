/*
* SYSTEM ACTION
* Kill a player and handle side effects
*/

use crate::{
    ID,
    action::{
        ActionActor, ActionError, ActionInterface, ActionResponse, Command, ResponseData,
        add_states::state_addition, get_actor, require_player,
    },
    actor::{ActorType, state::State},
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
    fn validate(&self, eng: &Engine, actor: &ActionActor) -> Result<(), ActionError> {
        actor.require_system()?;
        require_player(eng, self.target_id)?;

        let target = get_actor(eng, self.target_id)?;
        if target.states.contains(State::Dead) {
            return Err(ActionError::ActorIsDead);
        }

        if let Some(killer_id) = self.killer_id {
            get_actor(eng, killer_id)?;
        }

        Ok(())
    }

    fn execute(self, eng: &mut Engine, actor: &ActionActor) -> ActionResponse {
        let target = get_actor(eng, self.target_id).unwrap();
        let ActorType::Player(target_data) = &target.actor_type else {
            unreachable!()
        };
        let true_name = target_data.true_name.clone();

        // handle stuff like ability transfers, notebook transfers, etc...
        let mut next_actions = vec![state_addition(self.target_id, State::Dead)];
        if let Some(killer_id) = self.killer_id {
            let killer = get_actor(eng, killer_id).unwrap();
        }

        ActionResponse {
            commands: vec![Command::AnnounceDeath {
                true_name: String::from(&*true_name),
                death_message: if let Some(msg) = self.death_message {
                    msg
                } else {
                    String::from("Placeholder death message")
                },
                role: Role::Civilian,
                had_notebook: false,
            }],
            next_actions,
            data: ResponseData::Kill(KillResponse {}),
        }
    }
}
