/*
* SYSTEM ACTION
* Add a new player to the world
*/

use crate::{
    ID,
    action::{
        Action, ActionActor, ActionContext, ActionError, ActionInterface, ActionResponse,
        ActionResult, ability::create_and_give_ability::CreateAndGiveAbility,
        actor::player::give_role::GiveRole, chargepool::add_charge_pool::AddChargePool,
    },
    common::Version,
    config::role::Role,
    engine::Engine,
    helpers::{get_actor_mut, get_charge_pool_mut},
};

#[derive(PartialEq, Eq, Clone)]
pub struct AddPlayerResponse {
    pub id: ID, // return the internal id assigned to this player
}

// true names must be unique
#[derive(PartialEq, Eq, Clone, Debug)]
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

        // player will only be physically created in the mutation path
        if mutate {
            // add pools BEFORE giving abilities (the pools must exist beforehand)
            let pools = eng.config.player_config.charge_pools.clone();
            for (name, specifier) in pools {
                let pool_response = Action::AddChargePool(AddChargePool {
                    base_charges: specifier.charges,
                    base_reset_time: specifier.reset_time,
                })
                .handle(eng, ctx, actor, version, mutate)?;
                let ActionResponse::AddChargePool(data) = pool_response else {
                    unreachable!()
                };
                let pool = get_charge_pool_mut(eng, data.id)?;
                pool.on_link();
                let player_actor = get_actor_mut(eng, player_id)?;
                player_actor.pool_map.insert(name, data.id);
            }

            let default_abilities = eng.config.defaults.universal_abilities.clone();
            for default_ability in default_abilities {
                Action::CreateAndGiveAbility(CreateAndGiveAbility {
                    ability_name: default_ability.name,
                    transferrable: false,
                    variant: default_ability.variant,
                    actor_id: player_id,
                    volatile: false,
                })
                .handle(eng, ctx, actor, version, mutate)?;
            }

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
