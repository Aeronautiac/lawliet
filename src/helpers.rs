use crate::{
    ID, Timestamp,
    ability::Ability,
    action::{Action, ActionActor, ActionError},
    actor::{Actor, ActorLinkType, ActorType, Player, restriction::Restriction, state::State},
    config::{
        ability::{AbilityConfig, AbilityIdentifier},
        role::{Role, RoleConfig},
    },
    engine::Engine,
    notebook::Notebook,
    passive::{Passive, PassiveType},
};

pub fn get_actor(eng: &Engine, actor_id: ID) -> Result<&Actor, ActionError> {
    let target = eng
        .world
        .get_actor(actor_id)
        .ok_or(ActionError::ActorNotFound)?;
    Ok(target)
}
pub fn get_actor_mut(eng: &mut Engine, actor_id: ID) -> Result<&mut Actor, ActionError> {
    let target = eng
        .world
        .get_actor_mut(actor_id)
        .ok_or(ActionError::ActorNotFound)?;
    Ok(target)
}
pub fn require_player(eng: &Engine, actor_id: ID) -> Result<(), ActionError> {
    let target = get_actor(eng, actor_id)?;
    if !matches!(target.actor_type, ActorType::Player(_)) {
        Err(ActionError::ActorIsNotPlayer)
    } else {
        Ok(())
    }
}
pub fn actor_id(actor: &ActionActor) -> Option<ID> {
    match actor {
        ActionActor::System => None,
        ActionActor::Player(id) => Some(*id),
        ActionActor::Organization(id) => Some(*id),
    }
}
pub fn require_time_not_passed(eng: &Engine, t: Timestamp) -> Result<(), ActionError> {
    if eng.is_future_timestamp(t) {
        Ok(())
    } else {
        Err(ActionError::TimeAlreadyPassed)
    }
}
pub fn require_alive(eng: &Engine, actor_id: ID) -> Result<(), ActionError> {
    require_player(eng, actor_id)?;
    let actor = get_actor(eng, actor_id)?;
    if actor.states.contains(State::Dead) {
        return Err(ActionError::ActorIsDead);
    }
    Ok(())
}
pub fn require_dead(eng: &Engine, actor_id: ID) -> Result<(), ActionError> {
    require_player(eng, actor_id)?;
    let actor = get_actor(eng, actor_id)?;
    if actor.states.contains(State::Dead) {
        return Ok(());
    }
    Err(ActionError::ActorIsAlive)
}
pub fn get_ability_mut(eng: &mut Engine, ability_id: ID) -> Result<&mut Ability, ActionError> {
    let target = eng
        .world
        .get_ability_mut(ability_id)
        .ok_or(ActionError::AbilityNotFound)?;
    Ok(target)
}

pub fn get_ability(eng: &Engine, ability_id: ID) -> Result<&Ability, ActionError> {
    let target = eng
        .world
        .get_ability(ability_id)
        .ok_or(ActionError::AbilityNotFound)?;
    Ok(target)
}

pub fn get_passive_mut(eng: &mut Engine, passive_id: ID) -> Result<&mut Passive, ActionError> {
    let target = eng
        .world
        .get_passive_mut(passive_id)
        .ok_or(ActionError::PassiveNotFound)?;
    Ok(target)
}

pub fn get_passive(eng: &Engine, passive_id: ID) -> Result<&Passive, ActionError> {
    let target = eng
        .world
        .get_passive(passive_id)
        .ok_or(ActionError::PassiveNotFound)?;
    Ok(target)
}

pub fn get_ability_config(eng: &Engine, ability: ID) -> Result<&AbilityConfig, ActionError> {
    let ability = get_ability(eng, ability)?;
    let target = eng.config.abilities.get(&AbilityIdentifier {
        name: ability.ability_name,
        variant: ability.variant,
    });
    if let Some(data) = target {
        Ok(data)
    } else {
        Err(ActionError::AbilityConfigNotFound)
    }
}

pub fn get_role_config(eng: &Engine, role: Role) -> Result<&RoleConfig, ActionError> {
    if let Some(role_config) = eng.config.roles.get(&role) {
        Ok(role_config)
    } else {
        Err(ActionError::RoleNotImplemented)
    }
}

pub fn actor_has_effective_passive(eng: &Engine, actor_id: ID, passive_type: PassiveType) -> bool {
    let Some(actor_data) = eng.world.get_actor(actor_id) else {
        return false;
    };
    for id in actor_data.passives.iter() {
        let passive = eng.world.get_passive(*id).unwrap(); // if the list is not accurate
        // to the passives that actually exist, then something is wrong with the engine and a crash
        // is warranted.
        if passive.ownership_struct.owner == Some(actor_id) && passive.passive_type == passive_type
        {
            return true;
        }
    }
    for link in &actor_data.actor_links {
        if link.link_type == ActorLinkType::Passive {
            let other_actor = get_actor(eng, link.link_dest).unwrap();
            if !other_actor.has_restriction(Restriction::PassiveLinks)
                && actor_has_effective_passive(eng, link.link_dest, passive_type)
            {
                return true;
            }
        }
    }
    false
}

pub fn get_player(eng: &Engine, id: ID) -> Result<&Player, ActionError> {
    let actor = get_actor(eng, id)?;
    if let ActorType::Player(player) = &actor.actor_type {
        Ok(player)
    } else {
        Err(ActionError::ActorIsNotPlayer)
    }
}

pub fn get_player_mut(eng: &mut Engine, id: ID) -> Result<&mut Player, ActionError> {
    let actor = get_actor_mut(eng, id)?;
    if let ActorType::Player(player) = &mut actor.actor_type {
        Ok(player)
    } else {
        Err(ActionError::ActorIsNotPlayer)
    }
}

pub fn get_notebook(eng: &Engine, id: ID) -> Result<&Notebook, ActionError> {
    let notebook = eng.world.get_notebook(id);
    if let Some(notebook_data) = notebook {
        Ok(notebook_data)
    } else {
        Err(ActionError::NotebookNotFound)
    }
}

pub fn get_notebook_mut(eng: &mut Engine, id: ID) -> Result<&mut Notebook, ActionError> {
    let notebook = eng.world.get_notebook_mut(id);
    if let Some(notebook_data) = notebook {
        Ok(notebook_data)
    } else {
        Err(ActionError::NotebookNotFound)
    }
}
