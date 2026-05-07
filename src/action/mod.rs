use enum_dispatch::enum_dispatch;

use crate::{
    ID,
    action::{
        ability::{
            add_ability::{AddAbility, AddAbilityResponse},
            clear_volatile_links::{ClearVolatileLinks, ClearVolatileLinksResponse},
            create_and_give_ability::{CreateAndGiveAbility, CreateAndGiveAbilityResponse},
            give_ability::{GiveAbility, GiveAbilityResponse},
            use_ability::{UseAbility, UseAbilityResponse},
        },
        actor::{
            add_state::{AddState, AddStateResponse},
            create_actor_links::{CreateActorLinks, CreateActorLinksResponse},
            org::{
                add_to_org::{AddToOrg, AddToOrgResponse},
                create_org::{CreateOrg, CreateOrgResponse},
                remove_from_org::{RemoveFromOrg, RemoveFromOrgResponse},
                use_org_ability::{UseOrgAbility, UseOrgAbilityResponse},
            },
            player::{
                add_player::{AddPlayer, AddPlayerResponse},
                give_role::{GiveRole, GiveRoleResponse},
                kill::{Kill, KillResponse},
                revive::{Revive, ReviveResponse},
                schedule_kill::{ScheduleKill, ScheduleKillResponse},
                schedule_revive::{ScheduleRevive, ScheduleReviveResponse},
            },
            purge_volatiles::{PurgeVolatiles, PurgeVolatilesResponse},
            remove_state::{RemoveState, RemoveStateResponse},
            sever_links::{SeverLinks, SeverLinksResponse},
        },
        chargepool::{
            add_charge_pool::{AddChargePool, AddChargePoolResponse},
            try_delete_charge_pool::{TryDeleteChargePool, TryDeleteChargePoolResponse},
        },
        engine::{
            null::{Null, NullResponse},
            schedule_job::{ScheduleJob, ScheduleJobResponse},
        },
        notebook::{
            add_notebook::{AddNotebook, AddNotebookResponse},
            create_and_give_notebook::{CreateAndGiveNotebook, CreateAndGiveNotebookResponse},
            give_notebook::{GiveNotebook, GiveNotebookResponse},
            lend_notebook::{LendNotebook, LendNotebookResponse},
            notebook_scheduled_kill::{NotebookScheduledKill, NotebookScheduledKillResponse},
            return_dormant_books::{ReturnDormantBooks, ReturnDormantBooksResponse},
            set_books_dormant::{SetBooksDormant, SetBooksDormantResponse},
            set_borrowers_to_owners::{SetBorrowersToOwners, SetBorrowersToOwnersResponse},
            take_notebook::{TakeNotebook, TakeNotebookResponse},
            write_name::{WriteName, WriteNameResponse},
        },
        passive::{
            add_passive::{AddPassive, AddPassiveResponse},
            create_and_give_passive::{CreateAndGivePassive, CreateAndGivePassiveResponse},
            give_passive::{GivePassive, GivePassiveResponse},
        },
        poll::{
            add_vote::{AddVote, AddVoteResponse},
            create_poll::{CreatePoll, CreatePollReponse},
            poll_timeout::{PollTimeout, PollTimeoutResponse},
            remove_vote::{RemoveVote, RemoveVoteResponse},
            update_polls::{UpdatePolls, UpdatePollsResponse},
        },
        world::{
            initialize_world::{InitializeWorld, InitializeWorldResponse},
            update::{Update, UpdateResponse},
        },
    },
    command::Command,
    common::Version,
    engine::Engine,
};

pub mod ability;
pub mod actor;
pub mod channel;
pub mod chargepool;
pub mod engine;
pub mod notebook;
pub mod passive;
pub mod poll;
pub mod world;

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
    PollDoesntExist,
    InvalidVoter,
    NotAVoter,
    PlayerIsBlacklisted,
    OrgDoesntHaveLeadership,
    ActorAlreadyInOrg,
    PlayerNotInOrg,
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
#[derive(Debug)]
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
    UpdatePolls(UpdatePolls),
    CreatePoll(CreatePoll),
    PollTimeout(PollTimeout),
    ScheduleJob(ScheduleJob),
    AddVote(AddVote),
    RemoveVote(RemoveVote),
    AddToOrg(AddToOrg),
    RemoveFromOrg(RemoveFromOrg),
    CreateOrg(CreateOrg),
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
    TryDeleteChargePool(TryDeleteChargePoolResponse),
    InitializeWorld(InitializeWorldResponse),
    AddChargePool(AddChargePoolResponse),
    ClearVolatileLinks(ClearVolatileLinksResponse),
    UseOrgAbility(UseOrgAbilityResponse),
    Update(UpdateResponse),
    UpdatePolls(UpdatePollsResponse),
    CreatePoll(CreatePollReponse),
    PollTimeout(PollTimeoutResponse),
    ScheduleJob(ScheduleJobResponse),
    AddVote(AddVoteResponse),
    RemoveVote(RemoveVoteResponse),
    AddToOrg(AddToOrgResponse),
    RemoveFromOrg(RemoveFromOrgResponse),
    CreateOrg(CreateOrgResponse),
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
        let _ = Action::Update(Update {}).handle(eng, ctx, actor, version, true);
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
