/*
* SYSTEM ACTION
* Atomically create an ability and give it to an org
*/

use crate::{
    ID,
    action::{
        Action, ActionInterface, ActionResponse, ability::add_ability::AddAbility,
        actor::org::give_org_ability::GiveOrgAbility,
    },
    actor::organization::OrgAbility,
    common::Variant,
    config::ability::AbilityName,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CreateAndGiveOrgAbilityResponse {
    pub id: ID,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CreateAndGiveOrgAbility {
    pub ability_name: AbilityName,
    pub variant: Variant,
    pub org_id: ID,
    pub settings: OrgAbility,
}

impl ActionInterface for CreateAndGiveOrgAbility {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.require_system()?;

        let add_response = Action::AddAbility(AddAbility {
            ability_name: self.ability_name,
            variant: self.variant,
            transferrable: false,
        })
        .handle(eng, ctx, actor, version, mutate)?;
        let ActionResponse::AddAbility(add_response_data) = add_response else {
            unreachable!()
        };
        let id = add_response_data.id;

        if mutate {
            Action::GiveOrgAbility(GiveOrgAbility {
                org_id: self.org_id,
                ability_id: id,
                settings: self.settings.clone(),
            })
            .handle(eng, ctx, actor, version, mutate)?;
        }

        Ok(ActionResponse::CreateAndGiveOrgAbility(
            CreateAndGiveOrgAbilityResponse { id },
        ))
    }
}
