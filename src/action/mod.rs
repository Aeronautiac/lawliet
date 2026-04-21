use enum_dispatch::enum_dispatch;

use crate::{
    ID,
    action::{add_player::*, add_states::*, command::*, kill::*, revive::*},
    actor::{Actor, ActorType},
    engine::Engine,
};

pub mod add_player;
pub mod add_states;
pub mod command;
pub mod kill;
pub mod revive;

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
}

pub enum ResponseData {
    Kill(KillResponse),
    AddState(AddStateResponse),
    AddPlayer(AddPlayerResponse),
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
