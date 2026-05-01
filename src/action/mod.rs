use enum_dispatch::enum_dispatch;

use crate::{
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
    command::Command,
    common::Version,
    engine::Engine,
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
