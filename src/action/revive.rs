/*
* SYSTEM ACTION
* Revive a dead player
*/

use crate::{
    ID,
    action::{ActionActor, ActionError, ActionInterface, ActionResponse},
    engine::Engine,
};

#[derive(PartialEq, Eq, Clone)]
pub struct ReviveResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct Revive {
    pub target_id: ID,
}

impl ActionInterface for Revive {
    fn validate(&self, eng: &Engine, actor: &ActionActor) -> Result<(), ActionError> {
        Ok(())
    }

    fn execute(self, eng: &mut Engine, actor: &ActionActor) -> ActionResponse {
        unimplemented!()
    }
}
