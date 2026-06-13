/*
* ADMIN / SYSTEM / ABILITY-OWNER ACTION
* Release a kidnapped player.
*
* Authorization: actor must be authoritative, or own the kidnapping's source ability.
*
* On execution:
* - remove kidnapping record (before RemoveState so UpdateKidnapChannels sees it gone)
* - RemoveState(victim, State::Kidnapped)
* - DestroyChannel(channel, archive: true)
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
    helpers::{actor_owns_ability, get_kidnapping},
    kidnapping::KidnappingSource,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ReleaseKidnappingResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ReleaseKidnapping {
    pub kidnapping_id: KidnappingKey,
    pub forced: bool,
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
        let victim_id = kidnapping.victim;
        let channel_id = kidnapping.channel_id;

        let authorized = actor.is_authoritative()
            || matches!(kidnapping.source, KidnappingSource::Ability(ab) if actor_owns_ability(eng, actor, ab));
        if !authorized {
            return Err(ActionError::InsufficientPermissions);
        }

        if mutate {
            eng.world.remove_kidnapping(self.kidnapping_id);
        }

        Action::DestroyChannel(DestroyChannel {
            channel_id,
            archive: !self.forced,
        })
        .handle(eng, ctx, &ActionActor::System, version, mutate)?;

        Action::RemoveState(RemoveState {
            actor_id: victim_id,
            state: State::Kidnapped,
        })
        .handle(eng, ctx, &ActionActor::System, version, mutate)?;

        Ok(ActionResponse::ReleaseKidnapping(
            ReleaseKidnappingResponse {},
        ))
    }
}
