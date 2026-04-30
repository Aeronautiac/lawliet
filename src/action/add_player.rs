/*
* SYSTEM ACTION
* Add a new player to the world
*/

use crate::{
    ID,
    action::{
        Action, ActionActor, ActionContext, ActionError, ActionInterface, ActionResponse,
        ActionResult, add_ability::AddAbility, give_ability::GiveAbility, give_role::GiveRole,
    },
    common::Version,
    config::role::Role,
    engine::Engine,
};

#[derive(PartialEq, Eq, Clone)]
pub struct AddPlayerResponse {
    pub id: ID, // return the internal id assigned to this player
}

// true names must be unique
#[derive(PartialEq, Eq, Clone)]
pub struct AddPlayer {
    pub true_name: String,
    pub starting_role: Role,
}

impl ActionInterface for AddPlayer {
    fn handle(
        &mut self,
        eng: &mut Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: Version,
        mutate: bool,
    ) -> ActionResult {
        actor.require_system()?;

        if eng.world.get_player_id_by_name(&self.true_name).is_some() {
            return Err(ActionError::NameNotUnique);
        }

        let player_id = if mutate {
            eng.world
                .add_player(&self.true_name, self.starting_role)
                .unwrap()
        } else {
            0
        };

        let mut ability_ids = vec![];
        let default_abilities = eng.config.defaults.universal_abilities.clone();
        for default_ability in default_abilities {
            let response = Action::AddAbility(AddAbility {
                volatile: false,
                ability_name: default_ability.name,
                transferrable: false,
                variant: default_ability.variant,
            })
            .handle(eng, ctx, actor, version, mutate)?;
            let ActionResponse::AddAbility(response_data) = response else {
                unreachable!()
            };
            let ability_id = response_data.id;
            ability_ids.push(ability_id);
        }

        if mutate {
            // abilities will only be physically created in the mutation path
            for ability in ability_ids {
                Action::GiveAbility(GiveAbility {
                    ability_id: ability,
                    actor_id: player_id,
                })
                .handle(eng, ctx, actor, version, mutate)?;
            }

            // giving a role only works if the player exists which only happens in the mutation path
            Action::GiveRole(GiveRole {
                target_id: player_id,
                role: self.starting_role,
            })
            .handle(eng, ctx, actor, version, mutate)?;
        }

        Ok(ActionResponse::AddPlayer(AddPlayerResponse {
            id: player_id,
        }))
    }
}
