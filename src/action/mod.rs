use enum_dispatch::enum_dispatch;

use crate::{
    ID, Timestamp,
    action::{
        add_notebook::{AddNotebook, AddNotebookResponse},
        add_player::*,
        add_state::*,
        create_ability_links::{CreateAbilityLinks, CreateAbilityLinksResponse},
        give_notebook::{GiveNotebook, GiveNotebookResponse},
        give_role::{GiveRole, GiveRoleResponse},
        kill::*,
        lend_notebook::{LendNotebook, LendNotebookResponse},
        remove_state::{RemoveState, RemoveStateResponse},
        revive::*,
        schedule_kill::{ScheduleKill, ScheduleKillResponse},
        write_name::{WriteName, WriteNameResponse},
    },
    actor::{Actor, ActorType, state::State},
    command::Command,
    common::Version,
    engine::Engine,
};

pub mod add_notebook;
pub mod add_player;
pub mod add_state;
pub mod create_ability_links;
pub mod give_notebook;
pub mod give_role;
pub mod kill;
pub mod lend_notebook;
pub mod remove_state;
pub mod revive;
pub mod schedule_kill;
pub mod write_name;

pub struct ActionResponse {
    pub commands: Vec<Command>,
    pub next_actions: Vec<Action>,
    pub data: ResponseData,
}

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
}

pub type ActionResult = Result<ActionResponse, ActionError>;

#[enum_dispatch]
pub trait ActionInterface {
    /// next_actions must not depend on state mutations performed by the action itself.
    fn handle(
        &mut self,
        eng: &mut Engine,
        actor: &ActionActor,
        version: Version,
        mutate: bool,
    ) -> ActionResult;
    // PROBLEM:
    // This prevents the pattern of:
    // - adding a new item to the world
    // - modifying the item which was just created
    // This means:
    // - creating a role, creating the abilities with that role, and linking the newly created
    // ability to some existing ability will fail if it checks for ability existence (because adding
    // an ability is a state mutation)
    // - a solution to this is to make it impossible for the link action to return an error and
    // simply just make it do nothing if either ability doesnt exist, but this is a bandaid and
    // there may be other scenarios where this same pattern is necessary
    // - another solution is to have some shared handler function which does the state mutation and
    // just call it directly from both actions
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
}

pub enum ResponseData {
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
}

impl Action {
    pub fn dry_run(
        &mut self,
        eng: &mut Engine,
        actor: &ActionActor,
        version: Version,
    ) -> ActionResult {
        self.handle(eng, actor, version, false)
    }

    pub fn execute(
        &mut self,
        eng: &mut Engine,
        actor: &ActionActor,
        version: Version,
    ) -> ActionResult {
        self.handle(eng, actor, version, true)
    }
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
