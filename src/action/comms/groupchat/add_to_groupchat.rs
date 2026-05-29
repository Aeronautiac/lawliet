/*
* SYSTEM & PLAYER ACTION
* Add a player to a group chat
*/

use crate::{
    ID,
    action::{
        Action, ActionError, ActionInterface, ActionResponse,
        comms::groupchat::set_groupchat_owner::SetGroupchatOwner,
    },
    actor::modifier::Modifier,
    command::Command,
    helpers::{actor_id, get_actor, get_actor_mut, get_gc, get_gc_mut, get_player_mut},
};

// make sure to keep the player's caches up to date as well

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct AddToGroupchatResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct AddToGroupchat {
    pub groupchat_id: ID,
    pub player_id: ID,
    pub owner: bool,
}

impl ActionInterface for AddToGroupchat {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.player_or_system()?;

        let gc = get_gc(eng, self.groupchat_id)?;
        if actor.is_player() {
            let id = actor_id(actor).expect("expected valid actor id");
            if gc.owner != Some(id) {
                return Err(ActionError::NotTheOwner);
            }

            let actor_data = get_actor(eng, id).expect("expected valid actor");
            if actor_data.has_modifier(Modifier::NoContact) {
                return Err(ActionError::CannotContact);
            }
        }

        let target_data = get_actor_mut(eng, self.player_id)?;
        if target_data.has_modifier(Modifier::NoContact) {
            return Err(ActionError::CannotContact);
        }

        let gc = get_gc_mut(eng, self.groupchat_id)?;
        let channel_id = gc.channel_id;
        if mutate {
            gc.add_member(self.player_id);
        }

        let player_data = get_player_mut(eng, self.player_id)?;
        if mutate {
            player_data.add_groupchat(self.groupchat_id);
        }

        if self.owner {
            Action::SetGroupchatOwner(SetGroupchatOwner {
                groupchat_id: self.groupchat_id,
                owner: Some(self.player_id),
            })
            .handle(
                eng,
                ctx,
                &crate::action::ActionActor::System,
                version,
                mutate,
            )?;
        }

        ctx.push_cmd(
            Command::MapGc {
                gc_id: self.groupchat_id,
                channel_id,
            },
            Some(self.player_id),
            eng.time,
        );

        Ok(ActionResponse::AddToGroupchat(AddToGroupchatResponse {}))
    }
}
