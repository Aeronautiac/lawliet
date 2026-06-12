/*
* SYSTEM / ADMIN ACTION
* Kidnap a player: create the kidnapping object, channel, and apply State::Kidnapped.
*
* Preconditions:
* - kidnapper exists
* - victim exists, is a player, does not have NoPresence, does not have StrengthenedPresence
*
* On execution:
* - create channel (loggable)
* - add victim to channel (Send | View, displayed as Raw)
* - AddState(victim, State::Kidnapped)
* - store Kidnapping in world
*
* TODO: commands
*/

use crate::{
    ActorKey,
    action::{
        Action, ActionActor, ActionContext, ActionError, ActionInterface, ActionResponse,
        ActionResult,
        actor::add_state::AddState,
        comms::channel::create_channel::CreateChannel,
        kidnapping::update_kidnap_channels::UpdateKidnapChannels,
    },
    actor::modifier::Modifier,
    actor::state::State,
    common::{KidnappingKey, Version},
    engine::Engine,
    helpers::{get_actor, require_player},
    kidnapping::{Kidnapping, KidnappingType},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CreateKidnappingResponse {
    pub id: KidnappingKey,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CreateKidnapping {
    pub kidnapper_id: ActorKey,
    pub victim_id: ActorKey,
    pub kidnapping_type: KidnappingType,
}

impl ActionInterface for CreateKidnapping {
    fn handle(
        &mut self,
        eng: &mut Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: Version,
        mutate: bool,
    ) -> ActionResult {
        actor.admin_or_system()?;

        get_actor(eng, self.kidnapper_id)?;
        require_player(eng, self.victim_id)?;

        let victim = get_actor(eng, self.victim_id).expect("already validated");
        if victim.has_modifier(Modifier::NoPresence) {
            return Err(ActionError::UserNotPresent);
        }
        if victim.has_modifier(Modifier::StrengthenedPresence) {
            return Err(ActionError::ActorHasStrengthenedPresence);
        }

        let channel_response = Action::CreateChannel(CreateChannel { loggable: true }).handle(
            eng,
            ctx,
            &ActionActor::System,
            version,
            mutate,
        )?;
        let ActionResponse::CreateChannel(ch_data) = channel_response else {
            unreachable!()
        };
        let channel_id = ch_data.id;

        Action::AddState(AddState {
            actor_id: self.victim_id,
            state: State::Kidnapped,
        })
        .handle(eng, ctx, &ActionActor::System, version, mutate)?;

        let id = if mutate {
            eng.world.add_kidnapping(Kidnapping {
                victim: self.victim_id,
                channel_id,
                kidnapping_type: self.kidnapping_type,
                kidnapper: self.kidnapper_id,
            })
        } else {
            KidnappingKey::default()
        };

        Action::UpdateKidnapChannels(UpdateKidnapChannels {})
            .handle(eng, ctx, &ActionActor::System, version, mutate)?;

        Ok(ActionResponse::CreateKidnapping(CreateKidnappingResponse {
            id,
        }))
    }
}
