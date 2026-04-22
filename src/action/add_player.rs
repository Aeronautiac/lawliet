/*
* SYSTEM ACTION
* Add a new player to the world
*/

use crate::{
    ID,
    action::{ActionActor, ActionError, ActionInterface, ActionResponse, ResponseData},
    config::role::Role,
    engine::Engine,
};

#[derive(PartialEq, Eq, Clone)]
pub struct AddPlayerResponse {
    pub id: ID, // return the internal id assigned to this player
}

// true names must be unique
#[derive(PartialEq, Eq, Clone)]
pub struct AddPlayer {
    pub true_name: String,
    pub starting_role: Role,
}

impl ActionInterface for AddPlayer {
    fn validate(&self, eng: &Engine, actor: &ActionActor) -> Result<(), ActionError> {
        actor.require_system()?;
        if eng.world.get_player_id_by_name(&self.true_name).is_some() {
            return Err(ActionError::NameNotUnique);
        }
        Ok(())
    }

    // add player actor to the world, return the actor id
    fn execute(self, eng: &mut Engine, _: &ActionActor) -> ActionResponse {
        let id = eng
            .world
            .add_player(&self.true_name, self.starting_role)
            .unwrap();
        ActionResponse {
            commands: vec![],
            next_actions: vec![],
            data: ResponseData::AddPlayer(AddPlayerResponse { id }),
        }
    }
}
