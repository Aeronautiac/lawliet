use enum_dispatch::enum_dispatch;

use crate::{
    ID,
    action::{
        add_ability::{AddAbility, AddAbilityResponse},
        add_charge_pool::{AddChargePool, AddChargePoolResponse},
        add_notebook::{AddNotebook, AddNotebookResponse},
        add_passive::{AddPassive, AddPassiveResponse},
        add_player::*,
        add_state::*,
        clear_volatile_links::{ClearVolatileLinks, ClearVolatileLinksResponse},
        create_actor_links::{CreateActorLinks, CreateActorLinksResponse},
        create_and_give_ability::{CreateAndGiveAbility, CreateAndGiveAbilityResponse},
        create_and_give_notebook::{CreateAndGiveNotebook, CreateAndGiveNotebookResponse},
        create_and_give_passive::{CreateAndGivePassive, CreateAndGivePassiveResponse},
        give_ability::{GiveAbility, GiveAbilityResponse},
        give_notebook::{GiveNotebook, GiveNotebookResponse},
        give_passive::{GivePassive, GivePassiveResponse},
        give_role::{GiveRole, GiveRoleResponse},
        initialize_world::{InitializeWorld, InitializeWorldResponse},
        kill::*,
        lend_notebook::{LendNotebook, LendNotebookResponse},
        notebook_scheduled_kill::{NotebookScheduledKill, NotebookScheduledKillResponse},
        null::{Null, NullResponse},
        purge_volatiles::{PurgeVolatiles, PurgeVolatilesResponse},
        remove_state::{RemoveState, RemoveStateResponse},
        return_dormant_books::{ReturnDormantBooks, ReturnDormantBooksResponse},
        revive::*,
        schedule_kill::{ScheduleKill, ScheduleKillResponse},
        schedule_revive::{ScheduleRevive, ScheduleReviveResponse},
        set_books_dormant::{SetBooksDormant, SetBooksDormantResponse},
        set_borrowers_to_owners::{SetBorrowersToOwners, SetBorrowersToOwnersResponse},
        sever_links::{SeverLinks, SeverLinksResponse},
        take_notebook::{TakeNotebook, TakeNotebookResponse},
        try_delete_charge_pool::{TryDeleteChargePool, TryDeleteChargePoolResponse},
        update::{Update, UpdateResponse},
        use_ability::{UseAbility, UseAbilityResponse},
        use_org_ability::{UseOrgAbility, UseOrgAbilityResponse},
        write_name::{WriteName, WriteNameResponse},
    },
    command::Command,
    common::Version,
    engine::Engine,
};

pub mod add_ability;
pub mod add_charge_pool;
pub mod add_notebook;
pub mod add_passive;
pub mod add_player;
pub mod add_state;
pub mod clear_volatile_links;
pub mod create_actor_links;
pub mod create_and_give_ability;
pub mod create_and_give_notebook;
pub mod create_and_give_passive;
pub mod give_ability;
pub mod give_notebook;
pub mod give_passive;
pub mod give_role;
pub mod initialize_world;
pub mod kill;
pub mod lend_notebook;
pub mod notebook_scheduled_kill;
pub mod null;
pub mod purge_volatiles;
pub mod remove_state;
pub mod return_dormant_books;
pub mod revive;
pub mod schedule_kill;
pub mod schedule_revive;
pub mod set_books_dormant;
pub mod set_borrowers_to_owners;
pub mod sever_links;
pub mod take_notebook;
pub mod try_delete_charge_pool;
pub mod update;
pub mod use_ability;
pub mod use_org_ability;
pub mod write_name;

#[derive(Debug)]
pub enum ActionError {
    ActorNotFound,
    ActorIsDead,
    ActorIsAlive,
    ActorHasNotebookReceiveRestriction,
    InsufficientPermissions,
    ActorIsNotPlayer,
    NameNotUnique,
    NotebookNotFound,
    NotebookNotOwned,
    NotebookUsageBlocked, // later have it hold a vector of reasons/states
    NotebookPassageBlocked,
    NotebookOnCooldown,
    CannotLendToYourself,
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
    ItemAlreadyUnowned,
    ChargePoolNotFound,
    ActorIsNotOrg,
    PlayerIsNotLeader,
}

pub type ActionResult = Result<ActionResponse, ActionError>;

pub struct ActionContext {
    pub commands: Vec<Command>,
}

#[enum_dispatch]
pub trait ActionInterface {
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
    TakeNotebook(TakeNotebook),
    Null(Null),
    SetBorrowersToOwners(SetBorrowersToOwners),
    SetBooksDormant(SetBooksDormant),
    ReturnDormantBooks(ReturnDormantBooks),
    NotebookScheduledKill(NotebookScheduledKill),
    TryDeleteChargePool(TryDeleteChargePool),
    InitializeWorld(InitializeWorld),
    AddChargePool(AddChargePool),
    ClearVolatileLinks(ClearVolatileLinks),
    UseOrgAbility(UseOrgAbility),
    Update(Update),
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
    TakeNotebook(TakeNotebookResponse),
    Null(NullResponse),
    SetBorrowersToOwners(SetBorrowersToOwnersResponse),
    SetBooksDormant(SetBooksDormantResponse),
    ReturnDormantBooks(ReturnDormantBooksResponse),
    NotebookScheduledKill(NotebookScheduledKillResponse),
    DeleteChargePool(TryDeleteChargePoolResponse),
    InitializeWorld(InitializeWorldResponse),
    AddChargePool(AddChargePoolResponse),
    ClearVolatileLinks(ClearVolatileLinksResponse),
    UseOrgAbility(UseOrgAbilityResponse),
    Update(UpdateResponse),
}

#[derive(PartialEq, Eq, Clone)]
pub struct OrgActorInfo {
    pub org_id: ID,
    pub player_id: ID,
}

#[derive(PartialEq, Eq, Clone)]
pub enum ActionActor {
    System,
    Player(crate::ID),
    Organization(OrgActorInfo),
}

#[derive(PartialEq, Eq, Clone)]
pub struct ActionRequest {
    pub actor: ActionActor,
    pub timestamp: crate::Time,
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

    pub fn org_only(&self) -> Result<(), ActionError> {
        if matches!(self, ActionActor::Organization(_)) {
            Ok(())
        } else {
            Err(ActionError::ActorIsNotOrg)
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

impl Action {
    pub fn execute(
        &mut self,
        eng: &mut Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: Version,
    ) -> ActionResult {
        let result = self.handle(eng, ctx, actor, version, true);
        Action::Update(Update {}).handle(eng, ctx, actor, version, true);
        result
    }

    pub fn validate(
        &mut self,
        eng: &mut Engine,
        ctx: &mut ActionContext,
        actor: &ActionActor,
        version: Version,
    ) -> ActionResult {
        self.handle(eng, ctx, actor, version, false)
    }
}

#[cfg(test)]
mod action_tests {
    use super::*;
}
