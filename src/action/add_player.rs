/*
* Add a new player to the world
*/

use crate::{
    ID,
    action::{ActionActor, ActionError, ActionInterface, ActionResponse, ResponseData},
    actor::{Actor, ActorType},
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
        // check if name is unique (player insertion is O(n) - this can likely be optimized through
        // usage of a name set or similar)
        for player in eng.world.actors.values() {
            if !matches!(player.actor_type, ActorType::Player(_)) {
                continue;
            }
            let ActorType::Player(player_data) = &player.actor_type else {
                unreachable!()
            };
            if self.true_name == player_data.true_name {
                return Err(ActionError::NameNotUnique);
            }
        }
        Ok(())
    }

    // add player actor to the world, return the actor id
    fn execute(self, eng: &mut Engine, actor: &ActionActor) -> ActionResponse {
        let id = eng
            .world
            .add_actor(Actor::new_player(self.true_name, self.starting_role));
        ActionResponse {
            commands: vec![],
            next_actions: vec![],
            data: ResponseData::AddPlayer(AddPlayerResponse { id }),
        }
    }
}
