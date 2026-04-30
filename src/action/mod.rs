use enum_dispatch::enum_dispatch;

use crate::{
    ID, Timestamp,
    ability::Ability,
    action::{
        add_ability::{AddAbility, AddAbilityResponse},
        add_notebook::{AddNotebook, AddNotebookResponse},
        add_passive::{AddPassive, AddPassiveResponse},
        add_player::*,
        add_state::*,
        create_ability_links::{CreateAbilityLinks, CreateAbilityLinksResponse},
        create_actor_links::{CreateActorLinks, CreateActorLinksResponse},
        give_ability::{GiveAbility, GiveAbilityResponse},
        give_notebook::{GiveNotebook, GiveNotebookResponse},
        give_passive::{GivePassive, GivePassiveResponse},
        give_role::{GiveRole, GiveRoleResponse},
        kill::*,
        lend_notebook::{LendNotebook, LendNotebookResponse},
        remove_state::{RemoveState, RemoveStateResponse},
        revive::*,
        schedule_kill::{ScheduleKill, ScheduleKillResponse},
        schedule_revive::{ScheduleRevive, ScheduleReviveResponse},
        sever_links::{SeverLinks, SeverLinksResponse},
        use_ability::{UseAbility, UseAbilityResponse},
        write_name::{WriteName, WriteNameResponse},
    },
    actor::{Actor, ActorType, state::State},
    command::Command,
    common::Version,
    config::ability::{AbilityConfig, AbilityIdentifier},
    engine::Engine,
    passive::{Passive, PassiveType},
};

pub mod add_ability;
pub mod add_notebook;
pub mod add_passive;
pub mod add_player;
pub mod add_state;
pub mod create_ability_links;
pub mod create_actor_links;
pub mod give_ability;
pub mod give_notebook;
pub mod give_passive;
pub mod give_role;
pub mod kill;
pub mod lend_notebook;
pub mod remove_state;
pub mod revive;
pub mod schedule_kill;
pub mod schedule_revive;
pub mod sever_links;
pub mod use_ability;
pub mod write_name;

#[derive(Debug)]
pub enum ActionError {
    ActorNotFound,
    ActorIsDead,
    ActorIsAlive,
    InsufficientPermissions,
    ActorIsNotPlayer,
    NameNotUnique,
    NotebookNotFound,
    NotebookNotOwned,
    NotebookUsageBlocked, // later have it hold a vector of reasons/states
    NotebookPassageBlocked,
    NotebookOnCooldown,
    TimeAlreadyPassed,
    AbilityCategoryBlocked,
    AlreadyHadRole,
    AbilityConfigNotFound,
    AbilityNotFound,
    ActorIsSystem,
    AbilityNotOwned,
    AbilityMismatch,
    AbilityNotEnoughCharges,
}

pub type ActionResult = Result<ActionResponse, ActionError>;

pub struct ActionContext {
    pub commands: Vec<Command>,
}

#[enum_dispatch]
pub trait ActionInterface {
    /// next_actions must not depend on state mutations performed by the action itself.
    fn handle(
        &mut self,
        eng: &mut Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: Version,
        mutate: bool,
    ) -> ActionResult;
}

#[derive(PartialEq, Eq, Clone)]
#[enum_dispatch(ActionInterface)]
pub enum Action {
    Kill(Kill),
    AddState(AddState),
    Revive(Revive),
    AddPlayer(AddPlayer),
    AddNotebook(AddNotebook),
    GiveNotebook(GiveNotebook),
    WriteName(WriteName),
    LendNotebook(LendNotebook),
    ScheduleKill(ScheduleKill),
    RemoveState(RemoveState),
    GiveRole(GiveRole),
    CreateAbilityLinks(CreateAbilityLinks),
    AddAbility(AddAbility),
    UseAbility(UseAbility),
    ScheduleRevive(ScheduleRevive),
    GiveAbility(GiveAbility),
    AddPassive(AddPassive),
    GivePassive(GivePassive),
    SeverLinks(SeverLinks),
    CreateActorLinks(CreateActorLinks),
}

pub enum ActionResponse {
    Kill(KillResponse),
    AddState(AddStateResponse),
    AddPlayer(AddPlayerResponse),
    AddNotebook(AddNotebookResponse),
    GiveNotebook(GiveNotebookResponse),
    WriteName(WriteNameResponse),
    LendNotebook(LendNotebookResponse),
    RemoveState(RemoveStateResponse),
    Revive(ReviveResponse),
    ScheduleKill(ScheduleKillResponse),
    GiveRole(GiveRoleResponse),
    CreateAbilityLinks(CreateAbilityLinksResponse),
    AddAbility(AddAbilityResponse),
    GiveAbility(GiveAbilityResponse),
    UseAbility(UseAbilityResponse),
    ScheduleRevive(ScheduleReviveResponse),
    AddPassive(AddPassiveResponse),
    GivePassive(GivePassiveResponse),
    SeverLinks(SeverLinksResponse),
    CreateActorLinks(CreateActorLinksResponse),
}

#[derive(PartialEq, Eq, Clone)]
pub enum ActionActor {
    System,
    Player(crate::ID),
    Organization(crate::ID),
}

#[derive(PartialEq, Eq, Clone)]
pub struct ActionRequest {
    pub actor: ActionActor,
    pub timestamp: crate::Timestamp,
    pub payload: Action,
}

impl ActionActor {
    pub fn require_system(&self) -> Result<(), ActionError> {
        if matches!(self, ActionActor::System) {
            Ok(())
        } else {
            Err(ActionError::InsufficientPermissions)
        }
    }

    pub fn player_only(&self) -> Result<(), ActionError> {
        if matches!(self, ActionActor::Player(_)) {
            Ok(())
        } else {
            Err(ActionError::ActorIsNotPlayer)
        }
    }

    pub fn require_not_system(&self) -> Result<(), ActionError> {
        if matches!(self, ActionActor::System) {
            Ok(())
        } else {
            Err(ActionError::ActorIsSystem)
        }
    }
}

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

pub fn get_passive_mut(eng: &mut Engine, ability_id: ID) -> Result<&mut Passive, ActionError> {
    let target = eng
        .world
        .get_passive_mut(ability_id)
        .ok_or(ActionError::AbilityNotFound)?;
    Ok(target)
}

pub fn get_passive(eng: &Engine, ability_id: ID) -> Result<&Passive, ActionError> {
    let target = eng
        .world
        .get_passive(ability_id)
        .ok_or(ActionError::AbilityNotFound)?;
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

/// true if a matching passive is found with the actor id as the owner
/// O(n) - can likely be improved upon later
pub fn actor_has_effective_passive(eng: &Engine, actor_id: ID, passive_type: PassiveType) -> bool {
    for (_, passive) in eng.world.passives.iter() {
        if passive.ownership_struct.owner == Some(actor_id) && passive.passive_type == passive_type
        {
            return true;
        }
    }
    false
}
