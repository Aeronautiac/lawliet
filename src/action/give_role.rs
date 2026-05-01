/*
* SYSTEM ACTION
* Change a player's role and grant them abilities, notebooks, passives, and links associated with that role
* This operation will reset a player's role state regardless of if they already have the role
* Changing a player's role destroys any of their volatile resources
*/

use crate::{
    ID,
    action::{
        Action, ActionContext, ActionInterface, ActionResponse,
        create_actor_links::CreateActorLinks, create_and_give_ability::CreateAndGiveAbility,
        create_and_give_notebook::CreateAndGiveNotebook,
        create_and_give_passive::CreateAndGivePassive, get_player_mut, get_role_config,
        purge_volatiles::PurgeVolatiles, sever_links::SeverLinks,
    },
    config::role::Role,
};

#[derive(PartialEq, Eq, Clone)]
pub struct GiveRoleResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct GiveRole {
    pub target_id: ID,
    pub role: Role,
}

impl ActionInterface for GiveRole {
    fn handle(
        &mut self,
        eng: &mut crate::engine::Engine,
        ctx: &mut ActionContext,
        actor: &super::ActionActor,
        version: crate::common::Version,
        mutate: bool,
    ) -> super::ActionResult {
        actor.require_system()?;

        let player = get_player_mut(eng, self.target_id)?;
        if mutate {
            player.role = self.role;
        }

        Action::PurgeVolatiles(PurgeVolatiles {
            actor_id: self.target_id,
        })
        .handle(eng, ctx, actor, version, mutate)?;

        Action::SeverLinks(SeverLinks {
            actor_id: self.target_id,
        })
        .handle(eng, ctx, actor, version, mutate)?;

        let role_config = get_role_config(eng, self.role)?.clone();
        for ability in &role_config.abilities {
            Action::CreateAndGiveAbility(CreateAndGiveAbility {
                ability_name: ability.identifier.name,
                variant: ability.identifier.variant,
                transferrable: ability.transferrable,
                actor_id: self.target_id,
                volatile: true,
            })
            .handle(eng, ctx, actor, version, mutate)?;
        }

        for passive in &role_config.passives {
            Action::CreateAndGivePassive(CreateAndGivePassive {
                actor_id: self.target_id,
                passive_type: passive.passive_type,
                transferrable: passive.transferrable,
                volatile: true,
            })
            .handle(eng, ctx, actor, version, mutate)?;
        }

        for notebook in &role_config.notebooks {
            Action::CreateAndGiveNotebook(CreateAndGiveNotebook {
                fake: notebook.fake,
                volatile: true,
                actor_id: self.target_id,
            })
            .handle(eng, ctx, actor, version, mutate)?;
        }

        Action::CreateActorLinks(CreateActorLinks {}).handle(eng, ctx, actor, version, mutate)?;

        Ok(ActionResponse::GiveRole(GiveRoleResponse {}))
    }
}
