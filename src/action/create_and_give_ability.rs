/*
* SYSTEM ACTION
* Atomically create an ability and give it to an actor
*/

use crate::{
    ID,
    action::{
        Action, ActionInterface, ActionResponse, add_ability::AddAbility, give_ability::GiveAbility,
    },
    common::Variant,
    config::ability::AbilityName,
};

#[derive(PartialEq, Eq, Clone)]
pub struct CreateAndGiveAbilityResponse {
    pub id: ID,
}

#[derive(PartialEq, Eq, Clone)]
pub struct CreateAndGiveAbility {
    pub ability_name: AbilityName,
    pub transferrable: bool,
    pub variant: Variant,
    pub actor_id: ID,
    pub volatile: bool,
}

impl ActionInterface for CreateAndGiveAbility {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut super::ActionContext,
        actor: &super::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> super::ActionResult {
        actor.require_system()?;

        let add_response = Action::AddAbility(AddAbility {
            ability_name: self.ability_name,
            variant: self.variant,
            transferrable: self.transferrable,
        })
        .handle(eng, ctx, actor, version, mutate)?;
        let ActionResponse::AddAbility(add_response_data) = add_response else {
            // if it returns the wrong struct, then the engine is broken, and a crash is
            // warranted
            unreachable!()
        };

        if mutate {
            Action::GiveAbility(GiveAbility {
                ability_id: add_response_data.id,
                actor_id: self.actor_id,
                volatile: self.volatile,
            })
            .handle(eng, ctx, actor, version, mutate)?;
        }

        Ok(ActionResponse::CreateAndGiveAbility(
            CreateAndGiveAbilityResponse {
                id: add_response_data.id,
            },
        ))
    }
}
