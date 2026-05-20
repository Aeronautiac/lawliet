use crate::{
    ID, Time,
    ability::AbilityBehaviour,
    action::{
        Action, ActionActor, ActionRequest, ActionResponse, ActionResult,
        ability::{
            add_link::AddLink, clear_links::ClearLinks,
            create_and_give_ability::CreateAndGiveAbility, use_ability::UseAbility,
        },
        actor::{
            org::{
                add_to_org::AddToOrg, change_org_leader::ChangeOrgLeader, create_org::CreateOrg,
                remove_from_org::RemoveFromOrg, set_leadership::SetLeadership,
            },
            player::{add_player::AddPlayer, kill::Kill, revive::Revive},
        },
        chargepool::add_charge_pool::AddChargePool,
        engine::null::Null,
        notebook::{
            create_and_give_notebook::CreateAndGiveNotebook, lend_notebook::LendNotebook,
            write_name::WriteName,
        },
        passive::create_and_give_passive::CreateAndGivePassive,
        poll::{add_vote::AddVote, create_poll::CreatePoll, remove_vote::RemoveVote},
        world::initialize_world::InitializeWorld,
    },
    actor::organization::LeadershipTransferPolicies,
    chargepool::PoolLinkType,
    common::LinkWeight,
    config::{actor::organization::OrganizationName, role::Role},
    engine::{Engine, ExecutionResult},
    passive::PassiveType,
};

pub fn add_player(eng: &mut Engine, timestamp: Time, starting_role: Role, true_name: &str) -> ID {
    let data = eng
        .execute(ActionRequest {
            timestamp,
            actor: ActionActor::System,
            payload: Action::AddPlayer(AddPlayer {
                true_name: String::from(true_name),
                starting_role,
            }),
        })
        .unwrap()
        .0;
    let ActionResponse::AddPlayer(response) = data else {
        unreachable!()
    };
    response.id
}

pub fn quick_kill(
    eng: &mut Engine,
    timestamp: Time,
    allow_link_chaining: bool,
    sever_links: bool,
    set_books_dormant: bool,
    target: ID,
) {
    eng.execute(ActionRequest {
        timestamp,
        actor: ActionActor::System,
        payload: Action::Kill(Kill {
            target_id: target,
            killer_id: None,
            death_message: None,
            silent: true,
            set_books_dormant,
            allow_link_chaining,
            sever_links,
        }),
    })
    .unwrap();
}

pub fn quick_revive(eng: &mut Engine, timestamp: Time, ignore_links: bool, target: ID) {
    eng.execute(ActionRequest {
        timestamp,
        actor: ActionActor::System,
        payload: Action::Revive(Revive {
            target_id: target,
            ignore_links,
        }),
    })
    .unwrap();
}

pub fn quick_write(
    eng: &mut Engine,
    writer: ID,
    timestamp: Time,
    notebook_id: ID,
    true_name: &str,
    delay: Time,
) -> ActionResult {
    let result = eng.execute(ActionRequest {
        actor: ActionActor::Player(writer),
        timestamp,
        payload: Action::WriteName(WriteName {
            true_name: true_name.into(),
            death_message: None,
            notebook_id,
            delay,
        }),
    });
    match result {
        Ok(response) => Ok(response.0),
        Err(err) => Err(err),
    }
}

pub fn null_action(eng: &mut Engine, time: Time) {
    eng.execute(ActionRequest {
        actor: ActionActor::System,
        timestamp: time,
        payload: Action::Null(Null {}),
    })
    .unwrap();
}

pub fn quick_lend(eng: &mut Engine, time: Time, notebook_id: ID, player_lending: ID, lend_to: ID) {
    eng.execute(ActionRequest {
        actor: ActionActor::Player(player_lending),
        timestamp: time,
        payload: Action::LendNotebook(LendNotebook {
            notebook_id,
            target_id: lend_to,
        }),
    })
    .unwrap();
}

pub fn quick_notebook(eng: &mut Engine, time: Time, player: ID, fake: bool) -> ID {
    let data = eng
        .execute(ActionRequest {
            actor: ActionActor::System,
            timestamp: time,
            payload: Action::CreateAndGiveNotebook(CreateAndGiveNotebook {
                fake,
                actor_id: player,
                volatile: false,
            }),
        })
        .unwrap()
        .0;
    let ActionResponse::CreateAndGiveNotebook(response) = data else {
        unreachable!()
    };
    response.id
}

pub fn quick_passive(
    eng: &mut Engine,
    time: Time,
    player: ID,
    passive_type: PassiveType,
    transferrable: bool,
) -> ID {
    let data = eng
        .execute(ActionRequest {
            actor: ActionActor::System,
            timestamp: time,
            payload: Action::CreateAndGivePassive(CreateAndGivePassive {
                passive_type,
                transferrable,
                actor_id: player,
                volatile: false,
            }),
        })
        .unwrap()
        .0;
    let ActionResponse::CreateAndGivePassive(response) = data else {
        unreachable!()
    };
    response.id
}

pub fn create_poll(eng: &mut Engine, time: Time, action: CreatePoll) -> ID {
    let data = eng
        .execute(ActionRequest {
            actor: ActionActor::System,
            timestamp: time,
            payload: Action::CreatePoll(action),
        })
        .unwrap()
        .0;
    let ActionResponse::CreatePoll(response) = data else {
        unreachable!()
    };
    response.id
}

pub fn add_vote(
    eng: &mut Engine,
    time: Time,
    poll_id: ID,
    voter_id: ID,
    accept: bool,
) -> ExecutionResult {
    eng.execute(ActionRequest {
        actor: ActionActor::Player(voter_id),
        timestamp: time,
        payload: Action::AddVote(AddVote { poll_id, accept }),
    })
}

pub fn remove_vote(eng: &mut Engine, time: Time, poll_id: ID, voter_id: ID) -> ExecutionResult {
    eng.execute(ActionRequest {
        actor: ActionActor::Player(voter_id),
        timestamp: time,
        payload: Action::RemoveVote(RemoveVote { poll_id }),
    })
}

pub fn default_kill(id: ID) -> Action {
    Action::Kill(Kill {
        allow_link_chaining: true,
        death_message: None,
        killer_id: None,
        target_id: id,
        sever_links: true,
        silent: false,
        set_books_dormant: false,
    })
}

pub fn quick_ability(eng: &mut Engine, time: Time, args: CreateAndGiveAbility) -> ID {
    let data = eng
        .execute(ActionRequest {
            actor: ActionActor::System,
            timestamp: time,
            payload: Action::CreateAndGiveAbility(args),
        })
        .unwrap()
        .0;
    let ActionResponse::CreateAndGiveAbility(response) = data else {
        unreachable!()
    };
    response.id
}

pub fn use_ability(
    eng: &mut Engine,
    time: Time,
    user_id: ID,
    ability_id: ID,
    args: AbilityBehaviour,
) -> ExecutionResult {
    eng.execute(ActionRequest {
        actor: ActionActor::Player(user_id),
        timestamp: time,
        payload: Action::UseAbility(UseAbility {
            ability_id,
            ability_args: args,
        }),
    })
}

pub fn quick_pool(eng: &mut Engine, time: Time, args: AddChargePool) -> ID {
    let data = eng
        .execute(ActionRequest {
            actor: ActionActor::System,
            timestamp: time,
            payload: Action::AddChargePool(args),
        })
        .unwrap()
        .0;
    let ActionResponse::AddChargePool(response) = data else {
        unreachable!()
    };
    response.id
}

pub fn quick_link(
    eng: &mut Engine,
    time: Time,
    ability_id: ID,
    pool_id: ID,
    link_type: PoolLinkType,
    weight: LinkWeight,
) {
    eng.execute(ActionRequest {
        actor: ActionActor::System,
        timestamp: time,
        payload: Action::AddLink(AddLink {
            ability_id,
            pool_id,
            weight,
            link_type,
            volatile: false,
        }),
    })
    .unwrap();
}

pub fn quick_clear_links(eng: &mut Engine, time: Time, ability_id: ID) {
    eng.execute(ActionRequest {
        actor: ActionActor::System,
        timestamp: time,
        payload: Action::ClearLinks(ClearLinks { ability_id }),
    })
    .unwrap();
}

pub fn init_world(eng: &mut Engine) {
    eng.execute(ActionRequest {
        actor: ActionActor::System,
        timestamp: 0,
        payload: Action::InitializeWorld(InitializeWorld {}),
    })
    .unwrap();
}

pub fn add_org(eng: &mut Engine, time: Time, org: OrganizationName) -> ID {
    let data = eng
        .execute(ActionRequest {
            timestamp: time,
            actor: ActionActor::System,
            payload: Action::CreateOrg(CreateOrg { name: org }),
        })
        .unwrap()
        .0;
    let ActionResponse::CreateOrg(response) = data else {
        unreachable!()
    };
    response.id
}

pub fn add_to_org(eng: &mut Engine, time: Time, org: ID, actor: ID, leader: bool, og: bool) {
    eng.execute(ActionRequest {
        actor: ActionActor::System,
        timestamp: time,
        payload: Action::AddToOrg(AddToOrg {
            actor_id: actor,
            leader,
            og,
            org_id: org,
        }),
    })
    .unwrap();
}

pub fn remove_from_org(eng: &mut Engine, time: Time, org: ID, actor: ID) {
    eng.execute(ActionRequest {
        actor: ActionActor::System,
        timestamp: time,
        payload: Action::RemoveFromOrg(RemoveFromOrg {
            actor_id: actor,
            org_id: org,
        }),
    })
    .unwrap();
}

pub fn set_leadership(
    eng: &mut Engine,
    time: Time,
    org: ID,
    policies: Option<LeadershipTransferPolicies>,
) {
    eng.execute(ActionRequest {
        actor: ActionActor::System,
        timestamp: time,
        payload: Action::SetLeadership(SetLeadership {
            policies,
            org_id: org,
        }),
    })
    .unwrap();
}

pub fn change_leader(eng: &mut Engine, time: Time, org: ID, actor: Option<ID>) {
    eng.execute(ActionRequest {
        actor: ActionActor::System,
        timestamp: time,
        payload: Action::ChangeOrgLeader(ChangeOrgLeader {
            org_id: org,
            new_leader: actor,
        }),
    })
    .unwrap();
}
