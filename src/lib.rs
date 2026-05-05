/*
* lawliet
* a high performance deterministic headless engine written in rust for a multi-day death note social deduction game
* - process atomic actions
* - simulate a timeline
* - maintain an internal priority queue of future events for job scheduling
* - most actions are executed directly
* - actions may invoke other actions
* - adding an event to the priority queue requires an action
* - handle permissions
* - handle game init and configuration through a series of actions
* - handle pure game logic
*/

// config is dynamic and determines many aspects of game behaviour during action processing
// changing config is an action (this allows for hosts to tune config values while a game is running,
// and have it influence game behaviour immediately)
// lawliet begins with default config, but you may change aspects of config with different actions
//
// a player is a distinct stateful actor
//
// an organization is similar to a player
// it is an instance of an actor
// it is a group of players, and its behaviour is identified by its type similarly to how players
// have roles
//
// a role is an identifier and
// a role is associated with some set of abilities and passives in the config struct
// the presence of a role within an organization can influence what that organization can do
//
// abilities are distinct stateful objects
// abilities have rules and permissions
// abilities can be owned by different kinds of actors and can be transferred between actors
// depending on their properties
// abilities have an ability type which is simply an identifier
// abilities may have a variant to further narrow behaviour if necessary
// abilities have a "category" which dictates which restrictions may apply to them
//
// a state is a simple identity with a set of restrictions associated with it
// restrictions can be added without states, but adding a state will add the restrictions
// associated with that state in config
//
// a restriction blocks some specific permission or set of permissions
// actors have source maps of "sources" to restrictions. they utilize bitmaps for utility and minor
// performance gains.
// for example, "incarcerated": [ ALIVE | PHYSICAL | SUPERNATURAL ] is an example of a restriction
// mapping.
//
// yagami hosts multiple lawliet instances and communicates via ipc. it also acts as a persistence
// and routing layer.
// game state is not snapshotted. instead, it is reconstructed from a saved actions sequence.
// yagami stores actions into a buffer. when certain conditions are met (timer, significant action,
// full buffer, etc...) it flushes it to a postgres db.
// yagami is a multithreaded process
//
// a frontend sends action requests to yagami, and yagami sends back the result. if the action
// succeeded, a command buffer is sent back. a proper frontend uses these commands to render
// the game state meant for that specific player.
// - Frontend clients are dumb and respond only to commands and errors
// - Frontend servers handle routing and similar tasks
// - Response data structs are used internally (tests, sub-action return values, yagami)
// - Frontends must have host controls and game views

mod ability;
mod action;
mod actor;
mod chargepool;
mod command;
mod common;
mod config;
mod engine;
mod helpers;
mod notebook;
mod ownership;
mod passive;
mod world;

pub use common::{ID, Time};

// TODO:
// - Implement organizations. Idea: organizations hold passives, and every member gets a passive link to
// - Touch up the ability category enum (physical vs supernatural doesnt make sense)
// - Begin implementing every ability
// - Implement polls/votes
// - Implement channels
//    * Implement lounges
//    * Implement group chats
// - Implement bugs (simple message relayers with channel context filtering)
// - Implement news (likely just a special channel)
// the orgs they are in. if they leave the org, the link is severed.
// - Implement any necessary actions
// - Go through everything and implement frontend commands
// - Write extensive integration tests

// Organizations are not as generic as immediately thought
// Many have different rank structures and specific invite, kick, etc... mechanics.
// May need to do something similar to abilities where there is a generalized organization struct
// wrapping specific organization behaviour structs.
//
// There is only some specific stuff (invite and leader mechanics).

#[cfg(test)]
mod tests {
    use crate::{
        ID, Time,
        action::{
            Action, ActionActor, ActionRequest, ActionResponse, ActionResult,
            add_player::AddPlayer, create_and_give_notebook::CreateAndGiveNotebook, kill::Kill,
            lend_notebook::LendNotebook, null::Null, revive::Revive, write_name::WriteName,
        },
        actor::state::State,
        config::role::Role,
        engine::Engine,
        helpers::{actor_has_effective_passive, get_actor, get_notebook},
        passive::{ContactLogType, PassiveType},
    };

    fn add_player(eng: &mut Engine, timestamp: Time, starting_role: Role, true_name: &str) -> ID {
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

    fn quick_kill(
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

    fn quick_revive(eng: &mut Engine, timestamp: Time, ignore_links: bool, target: ID) {
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

    fn quick_write(
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

    fn null_action(eng: &mut Engine, time: Time) {
        eng.execute(ActionRequest {
            actor: ActionActor::System,
            timestamp: time,
            payload: Action::Null(Null {}),
        })
        .unwrap();
    }

    fn quick_lend(eng: &mut Engine, time: Time, notebook_id: ID, player_lending: ID, lend_to: ID) {
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

    fn quick_notebook(eng: &mut Engine, time: Time, player: ID, fake: bool) -> ID {
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

    // Link behaviour:
    // Links are not severed if the death was caused by a link
    // If the death was not caused by a link, they are typically severed, though this can be
    // disabled as well
    #[test]
    fn actor_links() {
        let mut eng = Engine::new();

        let w_id_1 = add_player(&mut eng, 0, Role::Watari, "John Candlewick");
        let l_id = add_player(&mut eng, 3, Role::L, "John Pork");
        let w_id_2 = add_player(&mut eng, 5, Role::Watari, "Oima Haumzaundwich");

        assert!(actor_has_effective_passive(
            &eng,
            l_id,
            PassiveType::ContactLogs(ContactLogType::Full)
        ));

        // link to this one should be severed now
        quick_kill(&mut eng, 5, false, true, false, w_id_1);

        // L should still be linked to watari 1
        assert!(actor_has_effective_passive(
            &eng,
            l_id,
            PassiveType::ContactLogs(ContactLogType::Full)
        ));

        // this one should only kill watari 2 and L
        // links should remain intact
        quick_kill(&mut eng, 6, true, true, false, l_id);

        let watari1 = get_actor(&eng, w_id_1).unwrap();
        let watari2 = get_actor(&eng, w_id_2).unwrap();
        assert!(watari1.has_state(State::Dead) && watari2.has_state(State::Dead));

        // this one should only revive L
        quick_revive(&mut eng, 6, true, l_id);

        // the passive link to watari 2 should still be intact although disabled due to the passive
        // link restriction on watari 2
        assert!(!actor_has_effective_passive(
            &eng,
            l_id,
            PassiveType::ContactLogs(ContactLogType::Full)
        ));

        // links were ignored, so only L should have been revived
        let watari1 = get_actor(&eng, w_id_1).unwrap();
        let watari2 = get_actor(&eng, w_id_2).unwrap();
        assert!(watari1.has_state(State::Dead) && watari2.has_state(State::Dead));

        // kill L again, do not sever links, and do not allow chaining
        quick_kill(&mut eng, 6, false, false, false, l_id);

        // this should revive watari 2 along with L
        quick_revive(&mut eng, 6, false, l_id);

        // the passive link should be enabled again because there is no passive link restriction
        assert!(actor_has_effective_passive(
            &eng,
            l_id,
            PassiveType::ContactLogs(ContactLogType::Full)
        ));

        // only watari 2 and L should be revived as watari 1 died alone
        let watari1 = get_actor(&eng, w_id_1).unwrap();
        let watari2 = get_actor(&eng, w_id_2).unwrap();
        assert!(watari1.has_state(State::Dead) && !watari2.has_state(State::Dead));
    }

    #[test]
    fn basic_ability_usage() {}

    #[test]
    fn ability_links() {}

    #[test]
    fn ability_transfers() {}

    #[test]
    fn passive_transfers() {}

    // a fake notebook should not kill someone
    #[test]
    fn fake_notebook_write_delayed() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "Light Yagami");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "Quillsh Wammy");
        let notebook_id = quick_notebook(&mut eng, 0, p1, true);

        quick_write(&mut eng, p1, 0, notebook_id, "quillsh wammy", 40).unwrap();
        null_action(&mut eng, 39);

        let p1_actor = get_actor(&eng, p1).unwrap();
        let p2_actor = get_actor(&eng, p2).unwrap();
        assert!(!p1_actor.has_state(State::Dead));
        assert!(!p2_actor.has_state(State::Dead));

        null_action(&mut eng, 40);

        let p1_actor = get_actor(&eng, p1).unwrap();
        let p2_actor = get_actor(&eng, p2).unwrap();
        assert!(!p1_actor.has_state(State::Dead));
        assert!(!p2_actor.has_state(State::Dead));
    }

    #[test]
    fn fake_notebook_write_instant() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "Light Yagami");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "Quillsh Wammy");
        let notebook_id = quick_notebook(&mut eng, 0, p1, true);

        quick_write(&mut eng, p1, 0, notebook_id, "quillsh wammy", 0).unwrap();

        let p1_actor = get_actor(&eng, p1).unwrap();
        let p2_actor = get_actor(&eng, p2).unwrap();
        assert!(!p1_actor.has_state(State::Dead));
        assert!(!p2_actor.has_state(State::Dead));
    }

    #[test]
    fn notebook_write_delayed() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "Light Yagami");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "Quillsh Wammy");
        let notebook_id = quick_notebook(&mut eng, 0, p1, false);

        quick_write(&mut eng, p1, 0, notebook_id, "quillsh wammy", 40).unwrap();
        null_action(&mut eng, 39);

        let p1_actor = get_actor(&eng, p1).unwrap();
        let p2_actor = get_actor(&eng, p2).unwrap();
        assert!(!p1_actor.has_state(State::Dead));
        assert!(!p2_actor.has_state(State::Dead));

        null_action(&mut eng, 40);

        let p1_actor = get_actor(&eng, p1).unwrap();
        let p2_actor = get_actor(&eng, p2).unwrap();
        assert!(!p1_actor.has_state(State::Dead));
        assert!(p2_actor.has_state(State::Dead));
    }

    #[test]
    fn notebook_write_instant() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "Light Yagami");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "Quillsh Wammy");
        let notebook_id = quick_notebook(&mut eng, 0, p1, false);

        quick_write(&mut eng, p1, 0, notebook_id, "quillSh wammy", 0).unwrap();

        let p1_actor = get_actor(&eng, p1).unwrap();
        let p2_actor = get_actor(&eng, p2).unwrap();
        assert!(!p1_actor.has_state(State::Dead));
        assert!(p2_actor.has_state(State::Dead));
    }

    // if you kill someone who is holding a notebook, you should get that notebook
    #[test]
    fn notebook_kill_wielder() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");
        let p1_notebook_id = quick_notebook(&mut eng, 0, p1, false);
        let p2_notebook_id = quick_notebook(&mut eng, 0, p2, false);

        quick_write(&mut eng, p1, 0, p1_notebook_id, "p2", 0).unwrap();

        let p1_actor = get_actor(&eng, p1).unwrap();
        let p2_actor = get_actor(&eng, p2).unwrap();
        assert!(p1_actor.has_notebook(p2_notebook_id));
        assert!(!p2_actor.has_notebook(p2_notebook_id));
    }

    // what happens if you kill yourself while you are the true owner of a notebook?
    // - you should remain as the true owner, but the notebook should be unusable because you're dead
    // - the game should not announce a notebook transfer
    #[test]
    fn notebook_suicide() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "Light Yagami");
        let notebook_id = quick_notebook(&mut eng, 0, p1, false);

        quick_write(&mut eng, p1, 0, notebook_id, "light yagami", 121).unwrap();
        null_action(&mut eng, 122);

        let p1_actor = get_actor(&eng, p1).unwrap();
        assert!(p1_actor.has_notebook(notebook_id));
        assert!(p1_actor.has_state(State::Dead));
    }

    #[test]
    fn notebook_lend() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");
        let p1_notebook_id_1 = quick_notebook(&mut eng, 0, p1, false);

        quick_lend(&mut eng, 0, p1_notebook_id_1, p1, p2);

        let p1_actor = get_actor(&eng, p1).unwrap();
        let p2_actor = get_actor(&eng, p2).unwrap();
        assert!(!p1_actor.has_notebook(p1_notebook_id_1));
        assert!(p2_actor.has_notebook(p1_notebook_id_1));
    }

    // General rules:
    // - If you kill a notebook wielder, and you are not the true owner of that notebook, then the
    // notebook should be given to you. It doesn't matter if you killed yourself or not.
    // - Notebook transfers are only announced if a death resulted in the CURRENT owner of a death
    // note changing, not the true owner.

    // what happens if you kill someone you're lending to?
    // - should get back early
    #[test]
    fn notebook_kill_lent_to() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");
        let p1_notebook_id_1 = quick_notebook(&mut eng, 0, p1, false);
        let p1_notebook_id_2 = quick_notebook(&mut eng, 0, p1, false);

        quick_lend(&mut eng, 0, p1_notebook_id_2, p1, p2);
        quick_write(&mut eng, p1, 0, p1_notebook_id_1, "p2", 0).unwrap();

        let p1_actor = get_actor(&eng, p1).unwrap();
        let p2_actor = get_actor(&eng, p2).unwrap();
        assert!(p1_actor.has_notebook(p1_notebook_id_2));
        assert!(!p2_actor.has_notebook(p1_notebook_id_2));
    }

    // what happens if you kill yourself while being lended to?
    // - the notebook should become yours but should become unusable because you are dead
    // - do not announce notebook transfer
    #[test]
    fn borrowed_notebook_suicide() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");
        let notebook_id = quick_notebook(&mut eng, 0, p1, false);

        quick_lend(&mut eng, 0, notebook_id, p1, p2);
        quick_write(&mut eng, p2, 0, notebook_id, "p2", 0).unwrap();

        let p1_actor = get_actor(&eng, p1).unwrap();
        let p2_actor = get_actor(&eng, p2).unwrap();
        let notebook = get_notebook(&eng, notebook_id).unwrap();
        assert!(!p1_actor.has_notebook(notebook_id));
        assert!(p2_actor.has_notebook(notebook_id));
        assert!(notebook.get_true_owner().unwrap() == p2);
    }

    // what happens if you kill someone who is lending to you?
    // what happens if the owner dies while the notebook is being lent out to someone?
    // - the person who is currently holding the notebook becomes the true owner
    // - do not announce a transfer
    #[test]
    fn borrowed_notebook_true_owner_died() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");
        let notebook_id = quick_notebook(&mut eng, 0, p1, false);

        quick_lend(&mut eng, 0, notebook_id, p1, p2);
        quick_kill(&mut eng, 0, true, true, false, p1);

        let p1_actor = get_actor(&eng, p1).unwrap();
        let p2_actor = get_actor(&eng, p2).unwrap();
        let notebook = get_notebook(&eng, notebook_id).unwrap();
        assert!(!p1_actor.has_notebook(notebook_id));
        assert!(p2_actor.has_notebook(notebook_id));
        assert!(notebook.get_true_owner().unwrap() == p2);
    }

    // what happens if the person borrowing your book dies before it returns and isnt killed by anyone?
    // - the notebook is lost (it no longer has an owner)
    // - do not announce a transfer
    #[test]
    fn borrowed_notebook_die_no_killer() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");
        let notebook_id = quick_notebook(&mut eng, 0, p1, false);

        quick_lend(&mut eng, 0, notebook_id, p1, p2);
        quick_kill(&mut eng, 0, true, true, false, p2);

        let p1_actor = get_actor(&eng, p1).unwrap();
        let p2_actor = get_actor(&eng, p2).unwrap();
        let notebook = get_notebook(&eng, notebook_id).unwrap();
        assert!(!p1_actor.has_notebook(notebook_id));
        assert!(!p2_actor.has_notebook(notebook_id));
        assert!(notebook.get_true_owner().is_none());
    }

    // it is possible to die before your scheduled notebook death through things like being executed
    // - the scheduled death should fail with no side effects
    #[test]
    fn notebook_die_before_scheduled() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let notebook_id = quick_notebook(&mut eng, 0, p1, false);

        quick_write(&mut eng, p1, 0, notebook_id, "p1", 10).unwrap();
        quick_kill(&mut eng, 1, true, true, false, p1);
        null_action(&mut eng, 11);
    }

    // what happens if a dead player kills a living player who owns a notebook through a scheduled
    // kill?
    // - the notebook goes to the dead player, but the dead player cannot use the notebook due to
    // restrictions
    #[test]
    fn notebook_dead_kill_living() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");
        let p1_notebook_id = quick_notebook(&mut eng, 0, p1, false);
        let p2_notebook_id = quick_notebook(&mut eng, 0, p2, false);

        quick_write(&mut eng, p1, 0, p1_notebook_id, "p2", 40).unwrap();
        quick_write(&mut eng, p2, 0, p2_notebook_id, "p1", 0).unwrap();
        null_action(&mut eng, 50);

        let p1_actor = get_actor(&eng, p1).unwrap();
        let p2_actor = get_actor(&eng, p2).unwrap();
        assert!(p1_actor.has_notebook(p1_notebook_id));
        assert!(p1_actor.has_notebook(p2_notebook_id));
        assert!(!p2_actor.has_notebook(p1_notebook_id));
        assert!(!p2_actor.has_notebook(p2_notebook_id));
    }

    // what happens when someone writes a name that has already been scheduled in a notebook?
    // - the actions cancel each other out (scheduled death is removed, actor does not die)
    #[test]
    fn notebook_collisions() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let book_1_id = quick_notebook(&mut eng, 0, p1, false);
        let book_2_id = quick_notebook(&mut eng, 0, p1, false);
        let book_3_id = quick_notebook(&mut eng, 0, p1, false);

        quick_write(&mut eng, p1, 0, book_1_id, "p1", 99).unwrap();
        quick_write(&mut eng, p1, 0, book_2_id, "p1", 0).unwrap();

        let p1_actor = get_actor(&eng, p1).unwrap();
        assert!(!p1_actor.has_state(State::Dead));

        quick_write(&mut eng, p1, 0, book_3_id, "p1", 0).unwrap();

        let p1_actor = get_actor(&eng, p1).unwrap();
        assert!(p1_actor.has_state(State::Dead));
    }

    #[test]
    fn notebook_dormancy() {
        let mut eng = Engine::new();
        let p1 = add_player(&mut eng, 0, Role::Civilian, "p1");
        let p2 = add_player(&mut eng, 0, Role::Civilian, "p2");
        let p3 = add_player(&mut eng, 0, Role::Civilian, "p3");
        let book_id = quick_notebook(&mut eng, 0, p1, false);

        quick_lend(&mut eng, 0, book_id, p1, p2);
        quick_kill(&mut eng, 0, true, true, true, p1);

        let notebook = get_notebook(&eng, book_id).unwrap();
        assert!(notebook.get_dormant_owner() == Some(p1));
        assert!(notebook.get_true_owner() == Some(p2));

        quick_lend(&mut eng, 0, book_id, p2, p3);

        let notebook = get_notebook(&eng, book_id).unwrap();
        assert!(notebook.owner == Some(p3));

        quick_revive(&mut eng, 0, false, p1);

        let notebook = get_notebook(&eng, book_id).unwrap();
        assert!(notebook.get_dormant_owner().is_none());
        assert!(notebook.get_true_owner() == Some(p1));
        assert!(notebook.owner == Some(p1));
    }

    // test the consistency of actor caches (things like owned ability sets)
    #[test]
    fn actor_caches() {}
}
