use serde::de::Visitor;

use crate::{
    ID, Time,
    action::{
        Action, ActionActor, ActionContext, ActionError, ActionRequest, ActionResponse,
        ActionResult,
        actor::player::{add_player::AddPlayer, kill::Kill, revive::Revive},
        engine::null::Null,
        notebook::{
            create_and_give_notebook::CreateAndGiveNotebook, lend_notebook::LendNotebook,
            write_name::WriteName,
        },
        poll::{add_vote::AddVote, create_poll::CreatePoll, remove_vote::RemoveVote},
    },
    config::role::Role,
    engine::Engine,
    poll::{Poll, PollPolicy, PollVisibility, VoterPolicy},
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
) -> Result<(ActionResponse, ActionContext), ActionError> {
    eng.execute(ActionRequest {
        actor: ActionActor::Player(voter_id),
        timestamp: time,
        payload: Action::AddVote(AddVote { poll_id, accept }),
    })
}

pub fn remove_vote(
    eng: &mut Engine,
    time: Time,
    poll_id: ID,
    voter_id: ID,
) -> Result<(ActionResponse, ActionContext), ActionError> {
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
