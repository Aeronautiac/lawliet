/*
* ADMIN / SYSTEM / KIDNAPPER ACTION
* Release a kidnapped player. The kidnapper or a host may trigger this.
*
* On execution:
* - archive the kidnapping channel
* - RemoveState(victim, State::Kidnapped)
* - remove kidnapping from world
*
* TODO: commands (reveal kidnapper identity if public kidnapping)
*/

use crate::{
    action::{
        Action, ActionActor, ActionContext, ActionError, ActionInterface, ActionResponse,
        ActionResult, actor::remove_state::RemoveState,
        comms::channel::destroy_channel::DestroyChannel,
    },
    actor::state::State,
    common::{KidnappingKey, Version},
    engine::Engine,
    helpers::{actor_id, get_kidnapping},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ReleaseKidnappingResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ReleaseKidnapping {
    pub kidnapping_id: KidnappingKey,
}

impl ActionInterface for ReleaseKidnapping {
    fn handle(
        &mut self,
        eng: &mut Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: Version,
        mutate: bool,
    ) -> ActionResult {
        let kidnapping = get_kidnapping(eng, self.kidnapping_id)?;
        let kidnapper = kidnapping.kidnapper;
        let victim_id = kidnapping.victim;
        let channel_id = kidnapping.channel_id;

        let authorized = actor.is_authoritative() || actor_id(actor) == Some(kidnapper);
        if !authorized {
            return Err(ActionError::InsufficientPermissions);
        }

        Action::DestroyChannel(DestroyChannel {
            channel_id,
            archive: true,
        })
        .handle(eng, ctx, &ActionActor::System, version, mutate)?;

        Action::RemoveState(RemoveState {
            actor_id: victim_id,
            state: State::Kidnapped,
        })
        .handle(eng, ctx, &ActionActor::System, version, mutate)?;

        if mutate {
            eng.world.remove_kidnapping(self.kidnapping_id);
        }

        Ok(ActionResponse::ReleaseKidnapping(
            ReleaseKidnappingResponse {},
        ))
    }
}
