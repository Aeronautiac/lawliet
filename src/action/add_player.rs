/*
* SYSTEM ACTION
* Add a new player to the world
*/

use crate::{
    ID,
    action::{
        ActionActor, ActionContext, ActionError, ActionInterface, ActionResponse, ActionResult,
    },
    common::Version,
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
    fn handle(
        &mut self,
        eng: &mut Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        _: Version,
        mutate: bool,
    ) -> ActionResult {
        actor.require_system()?;

        if eng.world.get_player_id_by_name(&self.true_name).is_some() {
            return Err(ActionError::NameNotUnique);
        }

        let id = if mutate {
            eng.world
                .add_player(&self.true_name, self.starting_role.clone())
                .unwrap()
        } else {
            0
        };

        Ok(ActionResponse::AddPlayer(AddPlayerResponse { id }))
    }
}
