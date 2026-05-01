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
        create_and_give_ability::{CreateAndGiveAbility, CreateAndGiveAbilityResponse},
        create_and_give_notebook::{CreateAndGiveNotebook, CreateAndGiveNotebookResponse},
        create_and_give_passive::{CreateAndGivePassive, CreateAndGivePassiveResponse},
        give_ability::{GiveAbility, GiveAbilityResponse},
        give_notebook::{GiveNotebook, GiveNotebookResponse},
        give_passive::{GivePassive, GivePassiveResponse},
        give_role::{GiveRole, GiveRoleResponse},
        kill::*,
        lend_notebook::{LendNotebook, LendNotebookResponse},
        purge_volatiles::{PurgeVolatiles, PurgeVolatilesResponse},
        remove_state::{RemoveState, RemoveStateResponse},
        revive::*,
        schedule_kill::{ScheduleKill, ScheduleKillResponse},
        schedule_revive::{ScheduleRevive, ScheduleReviveResponse},
        sever_links::{SeverLinks, SeverLinksResponse},
        use_ability::{UseAbility, UseAbilityResponse},
        write_name::{WriteName, WriteNameResponse},
    },
    actor::{Actor, ActorLinkType, ActorType, Player, state::State},
    command::Command,
    common::Version,
    config::{
        ability::{AbilityConfig, AbilityIdentifier},
        role::{Role, RoleConfig},
    },
    engine::Engine,
    notebook::Notebook,
    passive::{Passive, PassiveType},
};

pub mod add_ability;
pub mod add_notebook;
pub mod add_passive;
pub mod add_player;
pub mod add_state;
pub mod create_ability_links;
pub mod create_actor_links;
pub mod create_and_give_ability;
pub mod create_and_give_notebook;
pub mod create_and_give_passive;
pub mod give_ability;
pub mod give_notebook;
pub mod give_passive;
pub mod give_role;
pub mod kill;
pub mod lend_notebook;
pub mod purge_volatiles;
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
    PassiveNotFound,
    AbilityConfigNotFound,
    AbilityNotFound,
    ActorIsSystem,
    AbilityNotOwned,
    AbilityMismatch,
    AbilityNotEnoughCharges,
    RoleNotImplemented,
    ItemAlreadyOwned,
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
    PurgeVolatiles(PurgeVolatiles),
    CreateAndGiveAbility(CreateAndGiveAbility),
    CreateAndGiveNotebook(CreateAndGiveNotebook),
    CreateAndGivePassive(CreateAndGivePassive),
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
    PurgeVolatiles(PurgeVolatilesResponse),
    CreateAndGiveAbility(CreateAndGiveAbilityResponse),
    CreateAndGiveNotebook(CreateAndGiveNotebookResponse),
    CreateAndGivePassive(CreateAndGivePassiveResponse),
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
    dbg!(&actor_data.actor_links);
    for link in &actor_data.actor_links {
        if link.link_type == ActorLinkType::Passive
            && actor_has_effective_passive(eng, link.link_dest, passive_type)
        {
            return true;
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
