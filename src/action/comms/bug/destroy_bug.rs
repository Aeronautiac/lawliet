/*
* SYSTEM ACTION
* TODO: implement
*/

use crate::{
    action::{ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult},
    command::Command,
    common::BugKey,
    helpers::{get_bug, get_player_mut},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DestroyBugResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DestroyBug {
    pub bug_id: BugKey,
}

impl ActionInterface for DestroyBug {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        _version: crate::common::Version,
        mutate: bool,
    ) -> ActionResult {
        actor.admin_or_system()?;
        let bug = get_bug(eng, self.bug_id)?;

        let player =
            get_player_mut(eng, bug.target_id).expect("expected valid player as a bug target");
        if mutate {
            player.remove_bug(self.bug_id);
            eng.world.remove_bug(self.bug_id);
        }

        ctx.push_cmd(
            Command::DeleteBug {
                bug_id: self.bug_id,
            },
            None,
            eng.time,
        );

        Ok(ActionResponse::DestroyBug(DestroyBugResponse {}))
    }
}
