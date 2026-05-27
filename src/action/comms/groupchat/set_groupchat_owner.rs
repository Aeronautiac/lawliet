/*
* SYSTEM & PLAYER ACTION
* Set the owner of a group chat
*/

use crate::{
    ID,
    action::{ActionError, ActionInterface, ActionResponse},
    actor::modifier::Modifier,
    helpers::{actor_id, get_actor, get_gc_mut, get_player},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SetGroupchatOwnerResponse {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SetGroupchatOwner {
    pub groupchat_id: ID,
    pub owner: Option<ID>,
}

impl ActionInterface for SetGroupchatOwner {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.player_or_system()?;

        if let Some(owner_id) = self.owner {
            get_player(eng, owner_id)?;
            let data = get_actor(eng, owner_id)?;
            if actor.is_player() && data.has_modifier(Modifier::NoContact) {
                return Err(ActionError::CannotContact);
            }
        }

        let gc = get_gc_mut(eng, self.groupchat_id)?;
        if actor.is_player() {
            let id = actor_id(actor).expect("expected valid actor id");
            if gc.owner != Some(id) {
                return Err(ActionError::NotTheOwner);
            }
        }

        if mutate {
            gc.set_owner(self.owner);
        }

        // TODO:
        // alert owners of change

        Ok(ActionResponse::SetGroupchatOwner(
            SetGroupchatOwnerResponse {},
        ))
    }
}
