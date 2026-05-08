/*
* SYSTEM ACTION
* Create a new organization
* Use config to determine details
* Remember that an org is just a variant of an actor
*/

use crate::{
    ID,
    action::{
        Action, ActionInterface, ActionResponse,
        ability::create_and_give_ability::CreateAndGiveAbility,
        passive::create_and_give_passive::CreateAndGivePassive,
    },
    actor::organization::LeadershipStruct,
    config::actor::organization::OrganizationName,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CreateOrgResponse {
    pub id: ID,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CreateOrg {
    pub name: OrganizationName,
    pub leadership: Option<LeadershipStruct>,
}

impl ActionInterface for CreateOrg {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut crate::action::ActionContext,
        actor: &crate::action::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> crate::action::ActionResult {
        actor.require_system()?;

        let org_config = eng
            .config
            .org_config
            .get(&self.name)
            .expect("Organization unimplemented!");
        let abilities = org_config.abilities.clone();
        let passives = org_config.passives.clone();

        let id = if mutate {
            eng.world.add_org(self.name, self.leadership.clone())
        } else {
            0
        };

        if mutate {
            for ability in abilities {
                Action::CreateAndGiveAbility(CreateAndGiveAbility {
                    ability_name: ability.identifier.name,
                    variant: ability.identifier.variant,
                    transferrable: false,
                    actor_id: id,
                    volatile: true,
                })
                .handle(eng, ctx, actor, version, mutate)?;
            }

            for passive in passives {
                Action::CreateAndGivePassive(CreateAndGivePassive {
                    actor_id: id,
                    passive_type: passive,
                    transferrable: false,
                    volatile: true,
                })
                .handle(eng, ctx, actor, version, mutate)?;
            }
        }

        Ok(ActionResponse::CreateOrg(CreateOrgResponse { id }))
    }
}
