use crate::{
    Time,
    ability::Ability,
    action::{ActionActor, ActionError},
    actor::{
        Actor, ActorLinkType, ActorType, Organization, Player,
        modifier::{Modifier, Modifiers},
        state::State,
    },
    bug::Bug,
    channel::Channel,
    chargepool::ChargePool,
    command::{Command, CommandPayload, DeferredCommand},
    common::{
        AbilityKey, ActorKey, BugKey, ChannelKey, ChargePoolKey, GroupchatKey, LoungeKey,
        NotebookKey, PassiveKey, PollKey, PollWeight,
    },
    config::{
        ability::{AbilityConfig, AbilityIdentifier},
        role::{Role, RoleConfig},
    },
    engine::Engine,
    groupchat::Groupchat,
    lounge::Lounge,
    notebook::Notebook,
    passive::{Passive, PassiveType},
    poll::Poll,
};

pub fn get_actor(eng: &Engine, actor_id: ActorKey) -> Result<&Actor, ActionError> {
    let target = eng
        .world
        .get_actor(actor_id)
        .ok_or(ActionError::ActorNotFound)?;
    Ok(target)
}
pub fn get_actor_mut(eng: &mut Engine, actor_id: ActorKey) -> Result<&mut Actor, ActionError> {
    let target = eng
        .world
        .get_actor_mut(actor_id)
        .ok_or(ActionError::ActorNotFound)?;
    Ok(target)
}
pub fn require_player(eng: &Engine, actor_id: ActorKey) -> Result<(), ActionError> {
    let target = get_actor(eng, actor_id)?;
    if !matches!(target.actor_type, ActorType::Player(_)) {
        Err(ActionError::ActorIsNotPlayer)
    } else {
        Ok(())
    }
}

pub fn actor_id(actor: &ActionActor) -> Option<ActorKey> {
    match actor {
        ActionActor::System | ActionActor::Admin => None,
        ActionActor::Player(id) => Some(*id),
        ActionActor::Organization(org_info) => Some(org_info.org_id),
    }
}

pub fn player_id(actor: &ActionActor) -> Option<ActorKey> {
    match actor {
        ActionActor::System | ActionActor::Admin => None,
        ActionActor::Player(id) => Some(*id),
        ActionActor::Organization(org_info) => Some(org_info.player_id),
    }
}

pub fn require_time_not_passed(eng: &Engine, t: Time) -> Result<(), ActionError> {
    if eng.is_future_timestamp(t) {
        Ok(())
    } else {
        Err(ActionError::TimeAlreadyPassed)
    }
}

pub fn require_alive(eng: &Engine, actor_id: ActorKey) -> Result<(), ActionError> {
    require_player(eng, actor_id)?;
    let actor = get_actor(eng, actor_id)?;
    if actor.states.contains(State::Dead) {
        return Err(ActionError::ActorIsDead);
    }
    Ok(())
}

pub fn require_dead(eng: &Engine, actor_id: ActorKey) -> Result<(), ActionError> {
    require_player(eng, actor_id)?;
    let actor = get_actor(eng, actor_id)?;
    if actor.states.contains(State::Dead) {
        return Ok(());
    }
    Err(ActionError::ActorIsAlive)
}

pub fn get_ability_mut(
    eng: &mut Engine,
    ability_id: AbilityKey,
) -> Result<&mut Ability, ActionError> {
    let target = eng
        .world
        .get_ability_mut(ability_id)
        .ok_or(ActionError::AbilityNotFound)?;
    Ok(target)
}

pub fn get_ability(eng: &Engine, ability_id: AbilityKey) -> Result<&Ability, ActionError> {
    let target = eng
        .world
        .get_ability(ability_id)
        .ok_or(ActionError::AbilityNotFound)?;
    Ok(target)
}

pub fn get_passive_mut(
    eng: &mut Engine,
    passive_id: PassiveKey,
) -> Result<&mut Passive, ActionError> {
    let target = eng
        .world
        .get_passive_mut(passive_id)
        .ok_or(ActionError::PassiveNotFound)?;
    Ok(target)
}

pub fn get_passive(eng: &Engine, passive_id: PassiveKey) -> Result<&Passive, ActionError> {
    let target = eng
        .world
        .get_passive(passive_id)
        .ok_or(ActionError::PassiveNotFound)?;
    Ok(target)
}

pub fn get_ability_config(
    eng: &Engine,
    ability: AbilityKey,
) -> Result<&AbilityConfig, ActionError> {
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

pub fn actor_get_effective_passive(
    eng: &Engine,
    actor_id: ActorKey,
    check: impl Fn(&PassiveType) -> bool + Copy,
) -> Option<PassiveKey> {
    let actor_data = eng.world.get_actor(actor_id)?;
    for id in actor_data.passives.iter() {
        let passive = eng.world.get_passive(*id).unwrap(); // if the list is not accurate
        // to the passives that actually exist, then something is wrong with the engine and a crash
        // is warranted.
        if passive.ownership_struct.owner == Some(actor_id) && check(&passive.passive_type) {
            return Some(*id);
        }
    }
    for link in &actor_data.actor_links {
        if link.link_type == ActorLinkType::Passive {
            let other_actor = get_actor(eng, link.link_dest).unwrap();
            if let Some(found_id) = actor_get_effective_passive(eng, link.link_dest, check)
                && !other_actor.has_modifier(Modifier::DisablePassiveLinks)
            {
                return Some(found_id);
            };
        }
    }
    None
}

pub fn get_player(eng: &Engine, id: ActorKey) -> Result<&Player, ActionError> {
    let actor = get_actor(eng, id)?;
    if let ActorType::Player(player) = &actor.actor_type {
        Ok(player)
    } else {
        Err(ActionError::ActorIsNotPlayer)
    }
}

pub fn get_player_mut(eng: &mut Engine, id: ActorKey) -> Result<&mut Player, ActionError> {
    let actor = get_actor_mut(eng, id)?;
    if let ActorType::Player(player) = &mut actor.actor_type {
        Ok(player)
    } else {
        Err(ActionError::ActorIsNotPlayer)
    }
}

pub fn get_org_mut(eng: &mut Engine, id: ActorKey) -> Result<&mut Organization, ActionError> {
    let actor = get_actor_mut(eng, id)?;
    if let ActorType::Org(org) = &mut actor.actor_type {
        Ok(org)
    } else {
        Err(ActionError::ActorIsNotPlayer)
    }
}

pub fn get_org(eng: &Engine, id: ActorKey) -> Result<&Organization, ActionError> {
    let actor = get_actor(eng, id)?;
    if let ActorType::Org(org) = &actor.actor_type {
        Ok(org)
    } else {
        Err(ActionError::ActorIsNotPlayer)
    }
}

pub fn get_notebook(eng: &Engine, id: NotebookKey) -> Result<&Notebook, ActionError> {
    let notebook = eng.world.get_notebook(id);
    if let Some(notebook_data) = notebook {
        Ok(notebook_data)
    } else {
        Err(ActionError::NotebookNotFound)
    }
}

pub fn get_notebook_mut(eng: &mut Engine, id: NotebookKey) -> Result<&mut Notebook, ActionError> {
    let notebook = eng.world.get_notebook_mut(id);
    if let Some(notebook_data) = notebook {
        Ok(notebook_data)
    } else {
        Err(ActionError::NotebookNotFound)
    }
}

pub fn get_charge_pool(eng: &Engine, id: ChargePoolKey) -> Result<&ChargePool, ActionError> {
    let pool = eng.world.get_charge_pool(id);
    if let Some(data) = pool {
        Ok(data)
    } else {
        Err(ActionError::ChargePoolNotFound)
    }
}

pub fn get_charge_pool_mut(
    eng: &mut Engine,
    id: ChargePoolKey,
) -> Result<&mut ChargePool, ActionError> {
    let pool = eng.world.get_charge_pool_mut(id);
    if let Some(data) = pool {
        Ok(data)
    } else {
        Err(ActionError::ChargePoolNotFound)
    }
}

pub fn get_poll(eng: &Engine, id: PollKey) -> Result<&Poll, ActionError> {
    let poll = eng.world.get_poll(id);
    if let Some(data) = poll {
        Ok(data)
    } else {
        Err(ActionError::PollDoesntExist)
    }
}

pub fn get_poll_mut(eng: &mut Engine, id: PollKey) -> Result<&mut Poll, ActionError> {
    let poll = eng.world.get_poll_mut(id);
    if let Some(data) = poll {
        Ok(data)
    } else {
        Err(ActionError::PollDoesntExist)
    }
}

// return 0 for organizations, return 1 for normal players, return some other number if they have
// the vote amplification passive
pub fn get_voter_weight(eng: &Engine, id: ActorKey) -> PollWeight {
    get_actor(eng, id).expect("Expected a valid actor ID");
    if get_player(eng, id).is_ok() {
        let passive_id = actor_get_effective_passive(eng, id, |passive_type| {
            matches!(passive_type, PassiveType::VoteAmplication { multiplier: _ })
        });
        if let Some(id) = passive_id {
            let passive = get_passive(eng, id).expect("Expected passive to exist");
            let PassiveType::VoteAmplication { multiplier: val } = passive.passive_type else {
                unreachable!();
            };
            val
        } else {
            1
        }
    } else {
        0
    }
}

pub fn get_channel(eng: &Engine, id: ChannelKey) -> Result<&Channel, ActionError> {
    let channel = eng.world.get_channel(id);
    if let Some(data) = channel {
        Ok(data)
    } else {
        Err(ActionError::ChannelDoesntExist)
    }
}

pub fn get_channel_mut(eng: &mut Engine, id: ChannelKey) -> Result<&mut Channel, ActionError> {
    let channel = eng.world.get_channel_mut(id);
    if let Some(data) = channel {
        Ok(data)
    } else {
        Err(ActionError::ChannelDoesntExist)
    }
}

pub fn get_lounge(eng: &Engine, id: LoungeKey) -> Result<&Lounge, ActionError> {
    let lounge = eng.world.get_lounge(id);
    if let Some(data) = lounge {
        Ok(data)
    } else {
        Err(ActionError::LoungeDoesntExist)
    }
}

pub fn get_lounge_mut(eng: &mut Engine, id: LoungeKey) -> Result<&mut Lounge, ActionError> {
    let lounge = eng.world.get_lounge_mut(id);
    if let Some(data) = lounge {
        Ok(data)
    } else {
        Err(ActionError::LoungeDoesntExist)
    }
}

pub fn get_gc(eng: &Engine, id: GroupchatKey) -> Result<&Groupchat, ActionError> {
    let gc = eng.world.get_groupchat(id);
    if let Some(data) = gc {
        Ok(data)
    } else {
        Err(ActionError::GroupchatDoesntExist)
    }
}

pub fn get_gc_mut(eng: &mut Engine, id: GroupchatKey) -> Result<&mut Groupchat, ActionError> {
    let gc = eng.world.get_groupchat_mut(id);
    if let Some(data) = gc {
        Ok(data)
    } else {
        Err(ActionError::GroupchatDoesntExist)
    }
}

pub fn get_bug(eng: &Engine, id: BugKey) -> Result<&Bug, ActionError> {
    eng.world.get_bug(id).ok_or(ActionError::BugNotFound)
}

pub fn get_bug_mut(eng: &mut Engine, id: BugKey) -> Result<&mut Bug, ActionError> {
    eng.world.get_bug_mut(id).ok_or(ActionError::BugNotFound)
}

pub fn cmd_all_deferred(eng: &mut Engine, cmd: Command, blocking_modifiers: Modifiers) {
    let player_ids: Vec<ActorKey> = eng
        .world
        .actors
        .iter()
        .filter_map(|(id, actor)| matches!(actor.actor_type, ActorType::Player(_)).then_some(id))
        .collect();
    for id in player_ids {
        eng.deferred_commands.push(DeferredCommand {
            payload: CommandPayload {
                timestamp: eng.time,
                recipient: Some(id),
                cmd: cmd.clone(),
            },
            blocking_modifiers,
        });
    }
}
