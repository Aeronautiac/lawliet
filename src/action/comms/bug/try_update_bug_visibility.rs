/*
* SYSTEM ACTION
* When a bug ability changes ownership, update bug log visibility for the old and new owners.
* Loops over all bugs (including archived ones) associated with the ability.
* No-op if the ability is not a bug ability.
*/

use crate::{
    action::{ActionInterface, ActionResponse},
    common::{AbilityKey, ActorKey, BugKey},
    config::ability::AbilityName,
    helpers::get_ability,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct TryUpdateBugVisibilityResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct TryUpdateBugVisibility {
    pub ability_id: AbilityKey,
    pub old_owner: Option<ActorKey>,
    pub new_owner: ActorKey,
}

impl ActionInterface for TryUpdateBugVisibility {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        _ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        _version: crate::common::Version,
        _mutate: bool,
    ) -> crate::action::ActionResult {
        actor.admin_or_system()?;

        let ability = get_ability(eng, self.ability_id)?;
        if ability.ability_name != AbilityName::Bug {
            return Ok(ActionResponse::TryUpdateBugVisibility(
                TryUpdateBugVisibilityResponse {},
            ));
        }

        let _bug_ids: Vec<BugKey> = eng
            .world
            .bugs
            .iter()
            .filter(|(_, bug)| bug.ability_id == self.ability_id)
            .map(|(id, _)| id)
            .collect();

        for _bug_id in _bug_ids {
            // TODO: push HideBugLog to old_owner and ShowBugLog to new_owner
            // once the bug log communication protocol is implemented
        }

        Ok(ActionResponse::TryUpdateBugVisibility(
            TryUpdateBugVisibilityResponse {},
        ))
    }
}
