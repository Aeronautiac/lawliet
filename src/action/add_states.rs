/*
* SYSTEM ACTION
* Add states and any associated restrictions found in engine config to an actor
*/

use crate::{
    ID,
    action::{
        Action, ActionActor, ActionError, ActionInterface, ActionResponse, ResponseData, get_actor,
        get_actor_mut,
    },
    actor::state::State,
    engine::Engine,
};

#[derive(PartialEq, Eq, Clone)]
pub struct AddStateResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct AddState {
    pub actor_id: ID,
    pub state: State,
}

pub fn state_addition(actor_id: ID, state: State) -> Action {
    Action::AddState(AddState { actor_id, state })
}

impl ActionInterface for AddState {
    fn validate(&self, eng: &Engine, actor: &ActionActor) -> Result<(), ActionError> {
        actor.require_system()?;
        get_actor(eng, self.actor_id)?;
        Ok(())
    }

    fn execute(self, eng: &mut Engine, actor: &ActionActor) -> ActionResponse {
        let restrictions = eng
            .config
            .state_restrictions
            .get(&self.state)
            .cloned()
            .unwrap_or_default();

        let target = get_actor_mut(eng, self.actor_id).unwrap();
        target.add_state(self.state, restrictions);

        ActionResponse {
            commands: vec![],
            next_actions: vec![],
            data: ResponseData::AddState(AddStateResponse {}),
        }
    }
}
