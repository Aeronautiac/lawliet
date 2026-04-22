use enum_dispatch::enum_dispatch;

use crate::{
    ID,
    action::{
        add_notebook::{AddNotebook, AddNotebookResponse},
        add_player::*,
        add_states::*,
        command::*,
        give_notebook::{GiveNotebook, GiveNotebookResponse},
        kill::*,
        lend_notebook::{LendNotebook, LendNotebookResponse},
        revive::*,
        schedule_job::{ScheduleJob, ScheduleJobResponse},
        write_name::{WriteName, WriteNameResponse},
    },
    actor::{Actor, ActorType},
    engine::Engine,
};

pub mod add_notebook;
pub mod add_player;
pub mod add_states;
pub mod command;
pub mod give_notebook;
pub mod kill;
pub mod lend_notebook;
pub mod revive;
pub mod schedule_job;
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
    InsufficientPermissions,
    ActorIsNotPlayer,
    NameNotUnique,
    NotebookNotFound,
    NotebookNotOwned,
    NotebookUsageBlocked, // later have it hold a vector of reasons/states
    NotebookPassageBlocked,
    NotebookOnCooldown,
    TimestampAlreadyPassed,
}

pub type ActionResult = Result<ActionResponse, ActionError>;

#[enum_dispatch]
pub trait ActionInterface {
    fn validate(&self, eng: &Engine, actor: &ActionActor) -> Result<(), ActionError>;
    fn execute(self, eng: &mut Engine, actor: &ActionActor) -> ActionResponse; // consumes the action
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
    ScheduleJob(ScheduleJob),
}

pub enum ResponseData {
    Kill(KillResponse),
    AddState(AddStateResponse),
    AddPlayer(AddPlayerResponse),
    AddNotebook(AddNotebookResponse),
    GiveNotebook(GiveNotebookResponse),
    WriteName(WriteNameResponse),
    LendNotebook(LendNotebookResponse),
    ScheduleJob(ScheduleJobResponse),
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
        .actors
        .get(&actor_id)
        .ok_or(ActionError::ActorNotFound)?;
    Ok(target)
}

pub fn get_actor_mut(eng: &mut Engine, actor_id: ID) -> Result<&mut Actor, ActionError> {
    let target = eng
        .world
        .actors
        .get_mut(&actor_id)
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
