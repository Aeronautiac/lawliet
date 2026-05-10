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
//
// POLL IDEAS:
// - Polls should cancel themselves if the action attached to them is rejected (pass mutate false)
// - The poll create action will check the attached action as well to gate initial creation
// - Adding a vote to a poll should first evaluate the poll
// - There should be an update action which handles sub-actions like poll
// updates which should run to keep game state up to date for things that may seem unrelated but
// significantly effect world state. Just scheduling poll update actions every 10 seconds or so is
// both unfair and inefficient. For example, an actor may die pushing the game state into a place
// where a poll may pass and imprison someone, but since it didnt update, that person may be able to
// do something before they were imprisoned even though they should already be in prison
// - An update action should ALWAYS run after any other action (things like polls may
// change depending on the things that other actions do. for example, killing a member of kira's
// kingdom who voted no for a poll might push that poll into the passing threshold even though there
// was no direct update to the poll
// - On every update, polls should be evaluated and checked for validity
// - Additionally, polls should be updated when they are interacted with (it is not
// necessary to even call the update function directly in handlers which simply modify poll state
// because the update action will be called directly afterwards anyway)
// - Update actions are called only AFTER other actions because there can be no poll with no initial
// creation action, and padding both sides would lead to double updates between every event
// (pointless)

// Update actions should be called not in the engine, but in the action execute function
// Dry runs SHOULD NOT call poll updates, only execute actions
// Interleaving is not an issue because actions are atomic by nature

mod ability;
mod action;
mod actor;
mod channel;
mod chargepool;
mod command;
mod common;
mod config;
mod engine;
mod helpers;
mod notebook;
mod ownership;
mod passive;
mod poll;
mod test_helpers;
mod world;

pub use common::{ID, Time};

// TODO:
// - Test the ability system, poll system, and organization system
// - Implement channels
//    * Implement lounges
//    * Implement group chats
// - Implement bugs (simple message relayers with channel context filtering)
// - Implement news (likely just a special channel within the world struct)
// - Implement any necessary actions
// - Begin implementing every ability and write tests for them
// - Go through everything and implement frontend commands
// - Write extensive integration tests
// - Write yagami
// - Write ryuk (ratatui)
// - Write amane (web)

#[cfg(test)]
mod tests {
    use crate::{
        actor::state::State,
        config::role::Role,
        engine::Engine,
        helpers::{actor_get_effective_passive, get_actor},
        passive::{ContactLogType, PassiveType},
        test_helpers::*,
    };

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

        assert!(
            actor_get_effective_passive(&eng, l_id, |passive_type| {
                matches!(passive_type, PassiveType::ContactLogs(ContactLogType::Full))
            })
            .is_some()
        );

        // link to this one should be severed now
        quick_kill(&mut eng, 5, false, true, false, w_id_1);

        // L should still be linked to watari 1
        assert!(
            actor_get_effective_passive(&eng, l_id, |passive_type| {
                matches!(passive_type, PassiveType::ContactLogs(ContactLogType::Full))
            })
            .is_some()
        );

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
        assert!(
            actor_get_effective_passive(&eng, l_id, |passive_type| {
                matches!(passive_type, PassiveType::ContactLogs(ContactLogType::Full))
            })
            .is_none()
        );

        // links were ignored, so only L should have been revived
        let watari1 = get_actor(&eng, w_id_1).unwrap();
        let watari2 = get_actor(&eng, w_id_2).unwrap();
        assert!(watari1.has_state(State::Dead) && watari2.has_state(State::Dead));

        // kill L again, do not sever links, and do not allow chaining
        quick_kill(&mut eng, 6, false, false, false, l_id);

        // this should revive watari 2 along with L
        quick_revive(&mut eng, 6, false, l_id);

        // the passive link should be enabled again because there is no passive link restriction
        assert!(
            actor_get_effective_passive(&eng, l_id, |passive_type| {
                matches!(passive_type, PassiveType::ContactLogs(ContactLogType::Full))
            })
            .is_some()
        );

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

    // test the consistency of actor caches (things like owned ability sets)
    #[test]
    fn actor_caches() {}
}
