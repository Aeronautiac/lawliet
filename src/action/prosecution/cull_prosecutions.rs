/*
* SYSTEM ACTION
* Check all active prosecutions for forced termination conditions.
* Called from AddState and RemoveState, since all termination conditions are driven by
* state changes (NoPresence, death).
*
* Poll resolution is not handled here — when the prosecution poll concludes it calls a
* dedicated action to apply the verdict and terminate the prosecution.
*
* Custody or Trial phase:
*   if prosecutor or defendant has NoPresence → TerminateProsecution
*
* Voting phase:
*   if defendant is dead → TerminateProsecution
*/

use crate::{
    action::{
        Action, ActionActor, ActionContext, ActionInterface, ActionResponse, ActionResult,
        prosecution::terminate_prosecution::TerminateProsecution,
    },
    actor::{modifier::Modifier, state::State},
    common::{ProsecutionKey, Version},
    engine::Engine,
    helpers::get_actor,
    prosecution::ProsecutionPhase,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CullProsecutionsResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CullProsecutions {}

impl ActionInterface for CullProsecutions {
    fn handle(
        &mut self,
        eng: &mut Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: Version,
        mutate: bool,
    ) -> ActionResult {
        actor.admin_or_system()?;

        let to_terminate: Vec<ProsecutionKey> = eng
            .world
            .prosecutions
            .iter()
            .filter_map(|(key, prosecution)| {
                let prosecutor = get_actor(eng, prosecution.prosecutor)
                    .expect("prosecutor must be a valid actor");
                let defendant = get_actor(eng, prosecution.defense.defendant)
                    .expect("defendant must be a valid actor");

                let should_terminate = match &prosecution.phase {
                    ProsecutionPhase::Custody { .. } | ProsecutionPhase::Trial { .. } => {
                        prosecutor.has_modifier(Modifier::NoPresence)
                            || defendant.has_modifier(Modifier::NoPresence)
                    }
                    ProsecutionPhase::Voting { .. } => defendant.has_state(State::Dead),
                };

                should_terminate.then_some(key)
            })
            .collect();

        for prosecution_id in to_terminate {
            Action::TerminateProsecution(TerminateProsecution { prosecution_id })
                .handle(eng, ctx, &ActionActor::System, version, mutate)?;
        }

        Ok(ActionResponse::CullProsecutions(CullProsecutionsResponse {}))
    }
}
