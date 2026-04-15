use crate::{
    ID,
    action::{add_states::*, kill::*, revive::*},
    actor::Actor,
    engine::Engine,
};

pub mod add_states;
pub mod kill;
pub mod revive;

// command the frontend
pub enum Command {
    AnnounceDeath {},
}

pub enum ResponseData {
    Kill(KillData),
    AddState(AddStateData),
}

pub struct ActionResponse {
    pub commands: Vec<Command>,
    pub next_actions: Vec<Action>,
    pub data: ResponseData,
}

pub enum ActionError {
    ActorNotFound,
    ActorIsDead,
    InsufficientPermissions,
}

pub type ActionResult = Result<ActionResponse, ActionError>;

#[derive(PartialEq, Eq, Clone)]
pub enum Action {
    AddState(AddStateArgs),
    Kill(KillArgs),
    Revive(ReviveArgs),
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

pub fn get_actor(eng: &mut Engine, actor_id: ID) -> Result<&mut Actor, ActionError> {
    let target = eng
        .world
        .actors
        .get_mut(actor_id)
        .ok_or(ActionError::ActorNotFound)?;
    Ok(target)
}

pub fn action_dispatch(eng: &mut Engine, request: ActionRequest) -> ActionResult {
    match request.payload {
        Action::AddState(args) => add_state(eng, request.actor, args),
        Action::Kill(args) => kill(eng, request.actor, args),
        Action::Revive(args) => unimplemented!(),
    }
}
