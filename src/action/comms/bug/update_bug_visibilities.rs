/*
* Authoritative Action
* Loop through every bug, clear visibilities, evaluate new visibilities based on current
* conditions
*/

// should be called when:
// - ability ownership changes
// - a bug is created
// - an actor's state changes
//
// this can be optimized later, but just calling it in these cases massively simplifies things
// without having to put it into the update action

use smallvec::{SmallVec, smallvec};

use crate::{
    ActorKey,
    action::{ActionInterface, ActionResponse},
    actor::{ActorType, modifier::Modifier},
    bug::BugSource,
    command::Command,
    helpers::{actor_get_effective_passive, get_ability, get_actor},
    passive::PassiveType,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UpdateBugVisibilitiesResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UpdateBugVisibilities {}

impl ActionInterface for UpdateBugVisibilities {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        _version: crate::common::Version,
        _mutate: bool,
    ) -> crate::action::ActionResult {
        actor.admin_or_system()?;

        let mut custody_viewers: SmallVec<[ActorKey; 8]> = smallvec![];
        for (key, _) in eng.world.actors.iter().filter(|(_, act)| {
            matches!(act.actor_type, ActorType::Player(_))
                && !act.has_modifier(Modifier::NoPresence)
        }) {
            if actor_get_effective_passive(eng, key, |passive| {
                matches!(passive, PassiveType::CustodyBugReceiver)
            })
            .is_some()
            {
                custody_viewers.push(key)
            }
        }

        for (key, bug) in &eng.world.bugs {
            ctx.push_cmd(Command::ClearBugVisibily { bug_id: key }, None, eng.time);
            match &bug.source {
                BugSource::Ability(ability_id) => {
                    let ability = get_ability(eng, *ability_id)?;
                    if let Some(owner) = ability.ownership_struct.owner {
                        let actor = get_actor(eng, owner)?;
                        if !actor.has_modifier(Modifier::NoPresence) {
                            ctx.push_cmd(
                                Command::SetBugVisibility {
                                    bug_id: key,
                                    visible: true,
                                },
                                Some(owner),
                                eng.time,
                            );
                        }
                    }
                }
                BugSource::Custody => {
                    for id in &custody_viewers {
                        ctx.push_cmd(
                            Command::SetBugVisibility {
                                bug_id: key,
                                visible: true,
                            },
                            Some(*id),
                            eng.time,
                        );
                    }
                }
            }
        }

        Ok(ActionResponse::UpdateBugVisibilities(
            UpdateBugVisibilitiesResponse {},
        ))
    }
}
