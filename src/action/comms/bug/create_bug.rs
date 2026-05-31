/*
* SYSTEM ACTION
* Create a bug that relays messages from a target player to a bug log
*/

use crate::{
    ID,
    action::{ActionInterface, ActionResponse},
    bug::Bug,
    helpers::{get_ability, get_player_mut},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CreateBugResponse {
    pub id: ID,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CreateBug {
    pub target_id: ID,
    pub ability_id: ID,
}

impl ActionInterface for CreateBug {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        _ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        _version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.require_system()?;
        get_player_mut(eng, self.target_id)?;
        get_ability(eng, self.ability_id)?;

        let id = if mutate {
            let bug_id = eng.world.add_bug(Bug::new(self.target_id, self.ability_id));
            get_player_mut(eng, self.target_id)
                .expect("expected valid target player")
                .add_bug(bug_id);
            bug_id
        } else {
            0
        };

        Ok(ActionResponse::CreateBug(CreateBugResponse { id }))
    }
}
