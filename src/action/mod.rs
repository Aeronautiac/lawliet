use enum_dispatch::enum_dispatch;

use crate::{
    Time,
    action::{
        ability::{
            add_ability::{AddAbility, AddAbilityResponse},
            add_link::{AddLink, AddLinkResponse},
            clear_links::{ClearLinks, ClearLinksResponse},
            clear_volatile_links::{ClearVolatileLinks, ClearVolatileLinksResponse},
            create_and_give_ability::{CreateAndGiveAbility, CreateAndGiveAbilityResponse},
            destroy_ability::{DestroyAbility, DestroyAbilityResponse},
            give_ability::{GiveAbility, GiveAbilityResponse},
            remove_link::{RemoveLink, RemoveLinkResponse},
            use_ability::{UseAbility, UseAbilityResponse},
        },
        actor::{
            add_state::{AddState, AddStateResponse},
            create_actor_links::{CreateActorLinks, CreateActorLinksResponse},
            org::{
                add_to_org::{AddToOrg, AddToOrgResponse},
                change_org_leader::{ChangeOrgLeader, ChangeOrgLeaderResponse},
                create_and_give_org_ability::{
                    CreateAndGiveOrgAbility, CreateAndGiveOrgAbilityResponse,
                },
                create_org::{CreateOrg, CreateOrgResponse},
                give_org_ability::{GiveOrgAbility, GiveOrgAbilityResponse},
                remove_from_org::{RemoveFromOrg, RemoveFromOrgResponse},
                set_leadership::{SetLeadership, SetLeadershipResponse},
                system_use_org_ability::{SystemUseOrgAbility, SystemUseOrgAbilityResponse},
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
            add_charges::{AddCharges, AddChargesResponse},
            try_delete_charge_pool::{TryDeleteChargePool, TryDeleteChargePoolResponse},
        },
        comms::{
            bug::{
                archive_bug::{ArchiveBug, ArchiveBugResponse},
                create_bug::{CreateBug, CreateBugResponse},
                destroy_bug::{DestroyBug, DestroyBugResponse},
                update_bug_visibilities::{UpdateBugVisibilities, UpdateBugVisibilitiesResponse},
            },
            channel::{
                create_channel::{CreateChannel, CreateChannelResponse},
                destroy_channel::{DestroyChannel, DestroyChannelResponse},
                send_message::{SendMessage, SendMessageResponse},
                set_loggable::{SetLoggable, SetLoggableResponse},
                set_member::{SetMember, SetMemberResponse},
            },
            groupchat::{
                add_to_groupchat::{AddToGroupchat, AddToGroupchatResponse},
                create_groupchat::{CreateGroupchat, CreateGroupchatResponse},
                remove_from_groupchat::{RemoveFromGroupchat, RemoveFromGroupchatResponse},
                set_groupchat_owner::{SetGroupchatOwner, SetGroupchatOwnerResponse},
            },
            lounge::{
                create_lounge::{CreateLounge, CreateLoungeResponse},
                leave_lounge::{LeaveLounge, LeaveLoungeResponse},
                remove_from_lounge::{RemoveFromLounge, RemoveFromLoungeResponse},
            },
            update_contact_channels::{UpdateContactChannels, UpdateContactChannelsResponse},
        },
        engine::{
            deferred_cmds::{DeferredCmds, DeferredCmdsResponse},
            null::{Null, NullResponse},
            schedule_job::{ScheduleJob, ScheduleJobResponse},
        },
        notebook::{
            add_notebook::{AddNotebook, AddNotebookResponse},
            create_and_give_notebook::{CreateAndGiveNotebook, CreateAndGiveNotebookResponse},
            destroy_notebook::{DestroyNotebook, DestroyNotebookResponse},
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
            destroy_passive::{DestroyPassive, DestroyPassiveResponse},
            give_passive::{GivePassive, GivePassiveResponse},
        },
        poll::{
            add_vote::{AddVote, AddVoteResponse},
            create_poll::{CreatePoll, CreatePollReponse},
            poll_cleanup::{PollCleanup, PollCleanupResponse},
            poll_timeout::{PollTimeout, PollTimeoutResponse},
            remove_vote::{RemoveVote, RemoveVoteResponse},
            update_polls::{UpdatePolls, UpdatePollsResponse},
        },
        kidnapping::{
            create_kidnapping::{CreateKidnapping, CreateKidnappingResponse},
            release_kidnapping::{ReleaseKidnapping, ReleaseKidnappingResponse},
            update_kidnap_channels::{UpdateKidnapChannels, UpdateKidnapChannelsResponse},
        },
        prosecution::{
            advance_prosecution::{AdvanceProsecution, AdvanceProsecutionResponse},
            cull_prosecutions::{CullProsecutions, CullProsecutionsResponse},
            prosecution_vote_res::{ProsecutionVoteRes, ProsecutionVoteResResponse},
            select_lawyer::{SelectLawyer, SelectLawyerResponse},
            set_custody::{SetCustody, SetCustodyResponse},
            signal_ready::{SignalReady, SignalReadyResponse},
            start_prosecution::{StartProsecution, StartProsecutionResponse},
            terminate_prosecution::{TerminateProsecution, TerminateProsecutionResponse},
        },
        update::{Update, UpdateResponse},
        world::{
            add_to_world_channels::{AddToWorldChannels, AddToWorldChannelsResponse},
            create_orgs::{CreateOrgs, CreateOrgsResponse},
            initialize_engine::{InitializeEngine, InitializeEngineResponse},
            initialize_world::{InitializeWorld, InitializeWorldResponse},
            set_random_seed::{SetRandomSeed, SetRandomSeedResponse},
            set_world_channel_override::{
                SetWorldChannelOverride, SetWorldChannelOverrideResponse,
            },
            update_world_channel_perms::{
                UpdateWorldChannelPerms, UpdateWorldChannelPermsResponse,
            },
        },
    },
    command::{Command, CommandPayload},
    common::{ActorKey, Version},
    engine::Engine,
};

pub mod ability;
pub mod actor;
pub mod chargepool;
pub mod comms;
pub mod engine;
pub mod kidnapping;
pub mod notebook;
pub mod passive;
pub mod poll;
pub mod prosecution;
pub mod update;
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
    NotEnoughMembers,
    RequiredRolesNotPresent,
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
    AlreadyVoted,
    PlayerIsBlacklisted,
    OrgDoesntHaveLeadership,
    ActorAlreadyInOrg,
    UserNotPresent,
    PlayerNotInOrg,
    AlreadyLeader,
    ChannelDoesntExist,
    NotAChannelMember,
    DisplayNotOwned,
    PlayerNotInLounge,
    LoungeDoesntExist,
    GroupchatDoesntExist,
    CannotContact,
    NotTheOwner,
    NotInGroupchat,
    BugNotFound,
    ProsecutionNotFound,
    AlreadyADefendant,
    NotInProsecution,
    NotACustodyPhase,
    IncompatiblePhase,
    AlreadySignalled,
    LawyerAlreadySelected,
    CannotBeOwnLawyer,
    KidnappingNotFound,
    ActorHasStrengthenedPresence,
}

pub type ActionResult = Result<ActionResponse, ActionError>;

#[derive(Clone)]
pub struct ActionContext {
    pub commands: Vec<CommandPayload>,
}

impl ActionContext {
    pub fn push_cmd(&mut self, cmd: Command, recipient: Option<ActorKey>, time: Time) {
        self.commands.push(CommandPayload {
            timestamp: time,
            recipient,
            cmd,
        });
    }
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
    ChangeOrgLeader(ChangeOrgLeader),
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
    DestroyAbility(DestroyAbility),
    UseAbility(UseAbility),
    ScheduleRevive(ScheduleRevive),
    GiveAbility(GiveAbility),
    AddPassive(AddPassive),
    DestroyPassive(DestroyPassive),
    GivePassive(GivePassive),
    SeverLinks(SeverLinks),
    CreateActorLinks(CreateActorLinks),
    PurgeVolatiles(PurgeVolatiles),
    CreateAndGiveAbility(CreateAndGiveAbility),
    CreateAndGiveNotebook(CreateAndGiveNotebook),
    DestroyNotebook(DestroyNotebook),
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
    PollCleanup(PollCleanup),
    AddToOrg(AddToOrg),
    RemoveFromOrg(RemoveFromOrg),
    CreateOrg(CreateOrg),
    SystemUseOrgAbility(SystemUseOrgAbility),
    AddCharges(AddCharges),
    AddLink(AddLink),
    RemoveLink(RemoveLink),
    ClearLinks(ClearLinks),
    CreateOrgs(CreateOrgs),
    SetLeadership(SetLeadership),
    GiveOrgAbility(GiveOrgAbility),
    CreateAndGiveOrgAbility(CreateAndGiveOrgAbility),
    SendMessage(SendMessage),
    CreateChannel(CreateChannel),
    DestroyChannel(DestroyChannel),
    SetMember(SetMember),
    SetLoggable(SetLoggable),
    CreateLounge(CreateLounge),
    UpdateContactChannels(UpdateContactChannels),
    LeaveLounge(LeaveLounge),
    RemoveFromLounge(RemoveFromLounge),
    AddToGroupchat(AddToGroupchat),
    CreateGroupchat(CreateGroupchat),
    SetGroupchatOwner(SetGroupchatOwner),
    RemoveFromGroupchat(RemoveFromGroupchat),
    CreateBug(CreateBug),
    ArchiveBug(ArchiveBug),
    DestroyBug(DestroyBug),
    StartProsecution(StartProsecution),
    SetCustody(SetCustody),
    AdvanceProsecution(AdvanceProsecution),
    SignalReady(SignalReady),
    SelectLawyer(SelectLawyer),
    CullProsecutions(CullProsecutions),
    TerminateProsecution(TerminateProsecution),
    AddToWorldChannels(AddToWorldChannels),
    UpdateWorldChannelPerms(UpdateWorldChannelPerms),
    SetWorldChannelOverride(SetWorldChannelOverride),
    InitializeEngine(InitializeEngine),
    SetRandomSeed(SetRandomSeed),
    DeferredCmds(DeferredCmds),
    UpdateBugVisibilities(UpdateBugVisibilities),
    ProsecutionVoteRes(ProsecutionVoteRes),
    CreateKidnapping(CreateKidnapping),
    ReleaseKidnapping(ReleaseKidnapping),
    UpdateKidnapChannels(UpdateKidnapChannels),
}

pub enum ActionResponse {
    ChangeOrgLeader(ChangeOrgLeaderResponse),
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
    DestroyAbility(DestroyAbilityResponse),
    GiveAbility(GiveAbilityResponse),
    UseAbility(UseAbilityResponse),
    ScheduleRevive(ScheduleReviveResponse),
    AddPassive(AddPassiveResponse),
    DestroyPassive(DestroyPassiveResponse),
    GivePassive(GivePassiveResponse),
    SeverLinks(SeverLinksResponse),
    CreateActorLinks(CreateActorLinksResponse),
    PurgeVolatiles(PurgeVolatilesResponse),
    CreateAndGiveAbility(CreateAndGiveAbilityResponse),
    CreateAndGiveNotebook(CreateAndGiveNotebookResponse),
    DestroyNotebook(DestroyNotebookResponse),
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
    PollCleanup(PollCleanupResponse),
    AddToOrg(AddToOrgResponse),
    RemoveFromOrg(RemoveFromOrgResponse),
    CreateOrg(CreateOrgResponse),
    SystemUseOrgAbility(SystemUseOrgAbilityResponse),
    AddCharges(AddChargesResponse),
    AddLink(AddLinkResponse),
    RemoveLink(RemoveLinkResponse),
    ClearLinks(ClearLinksResponse),
    CreateOrgs(CreateOrgsResponse),
    SetLeadership(SetLeadershipResponse),
    GiveOrgAbility(GiveOrgAbilityResponse),
    CreateAndGiveOrgAbility(CreateAndGiveOrgAbilityResponse),
    SendMessage(SendMessageResponse),
    CreateChannel(CreateChannelResponse),
    DestroyChannel(DestroyChannelResponse),
    SetMember(SetMemberResponse),
    SetLoggable(SetLoggableResponse),
    CreateLounge(CreateLoungeResponse),
    UpdateContactChannels(UpdateContactChannelsResponse),
    LeaveLounge(LeaveLoungeResponse),
    RemoveFromLounge(RemoveFromLoungeResponse),
    AddToGroupchat(AddToGroupchatResponse),
    CreateGroupchat(CreateGroupchatResponse),
    SetGroupchatOwner(SetGroupchatOwnerResponse),
    RemoveFromGroupchat(RemoveFromGroupchatResponse),
    CreateBug(CreateBugResponse),
    ArchiveBug(ArchiveBugResponse),
    DestroyBug(DestroyBugResponse),
    StartProsecution(StartProsecutionResponse),
    SetCustody(SetCustodyResponse),
    AdvanceProsecution(AdvanceProsecutionResponse),
    SignalReady(SignalReadyResponse),
    SelectLawyer(SelectLawyerResponse),
    CullProsecutions(CullProsecutionsResponse),
    TerminateProsecution(TerminateProsecutionResponse),
    AddToWorldChannels(AddToWorldChannelsResponse),
    UpdateWorldChannelPerms(UpdateWorldChannelPermsResponse),
    SetWorldChannelOverride(SetWorldChannelOverrideResponse),
    InitializeEngine(InitializeEngineResponse),
    SetRandomSeed(SetRandomSeedResponse),
    DeferredCmds(DeferredCmdsResponse),
    UpdateBugVisibilities(UpdateBugVisibilitiesResponse),
    ProsecutionVoteRes(ProsecutionVoteResResponse),
    CreateKidnapping(CreateKidnappingResponse),
    ReleaseKidnapping(ReleaseKidnappingResponse),
    UpdateKidnapChannels(UpdateKidnapChannelsResponse),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct OrgActorInfo {
    pub org_id: ActorKey,
    pub player_id: ActorKey,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ActionActor {
    Admin,
    System,
    Player(ActorKey),
    Organization(OrgActorInfo),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ActionRequest {
    pub actor: ActionActor,
    pub timestamp: crate::Time,
    pub payload: Action,
}

impl ActionActor {
    pub fn require_system(&self) -> Result<(), ActionError> {
        if self.is_system() {
            Ok(())
        } else {
            Err(ActionError::InsufficientPermissions)
        }
    }

    pub fn admin_or_system(&self) -> Result<(), ActionError> {
        if self.is_admin() || self.is_system() {
            Ok(())
        } else {
            Err(ActionError::InsufficientPermissions)
        }
    }

    pub fn player_only(&self) -> Result<(), ActionError> {
        if self.is_player() {
            Ok(())
        } else {
            Err(ActionError::ActorIsNotPlayer)
        }
    }

    pub fn org_only(&self) -> Result<(), ActionError> {
        if self.is_org() {
            Ok(())
        } else {
            Err(ActionError::ActorIsNotOrg)
        }
    }

    pub fn require_not_system(&self) -> Result<(), ActionError> {
        if self.is_system() {
            Err(ActionError::ActorIsSystem)
        } else {
            Ok(())
        }
    }

    pub fn player_or_system(&self) -> Result<(), ActionError> {
        if !self.is_player() && !self.is_system() {
            Err(ActionError::InsufficientPermissions)
        } else {
            Ok(())
        }
    }

    pub fn player_or_authoritative(&self) -> Result<(), ActionError> {
        if !self.is_player() && !self.is_authoritative() {
            Err(ActionError::InsufficientPermissions)
        } else {
            Ok(())
        }
    }

    pub fn is_player(&self) -> bool {
        matches!(self, ActionActor::Player(_))
    }

    pub fn is_system(&self) -> bool {
        matches!(self, ActionActor::System)
    }

    pub fn is_admin(&self) -> bool {
        matches!(self, ActionActor::Admin)
    }

    pub fn is_authoritative(&self) -> bool {
        self.is_admin() || self.is_system()
    }

    pub fn is_org(&self) -> bool {
        matches!(self, ActionActor::Organization(_))
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
        Action::Update(Update {})
            .handle(eng, ctx, &ActionActor::System, version, true)
            .expect("Update action has failed");
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
