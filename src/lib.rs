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
// succeeded, a command buffer (filtered for the particular frontend it is being sent to by yagami) is
// sent back along with action specific metadata. a proper frontend uses these commands to render
// the game state meant for that specific player.

mod ability;
mod action;
mod actor;
mod command;
mod common;
mod config;
mod engine;
mod helpers;
mod notebook;
mod ownership;
mod passive;
mod world;

pub use common::{ID, Timestamp};

#[cfg(test)]
mod tests {
    use crate::{
        ID, Timestamp,
        action::{
            Action, ActionActor, ActionRequest, ActionResponse, add_player::AddPlayer, kill::Kill,
            revive::Revive,
        },
        actor::state::State,
        config::role::Role,
        engine::Engine,
        helpers::{actor_has_effective_passive, get_actor},
        passive::{ContactLogType, PassiveType},
    };

    fn add_player(
        eng: &mut Engine,
        timestamp: Timestamp,
        starting_role: Role,
        true_name: &str,
    ) -> ID {
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
        timestamp: Timestamp,
        allow_link_chaining: bool,
        sever_links: bool,
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
                allow_link_chaining,
                sever_links,
            }),
        })
        .unwrap();
    }

    fn quick_revive(eng: &mut Engine, timestamp: Timestamp, ignore_links: bool, target: ID) {
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

    // Link behaviour:
    // Links are not severed if the death was caused by a link
    // If the death was not caused by a link, they are typically severed, though this can be
    // disabled as well
    #[test]
    fn test_actor_links() {
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
        quick_kill(&mut eng, 5, false, true, w_id_1);

        // L should still be linked to watari 1
        assert!(actor_has_effective_passive(
            &eng,
            l_id,
            PassiveType::ContactLogs(ContactLogType::Full)
        ));

        // this one should only kill watari 2 and L
        // links should remain intact
        quick_kill(&mut eng, 6, true, true, l_id);

        let watari1 = get_actor(&eng, w_id_1).unwrap();
        let watari2 = get_actor(&eng, w_id_2).unwrap();
        assert!(watari1.states.contains(State::Dead) && watari2.states.contains(State::Dead));

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
        assert!(watari1.states.contains(State::Dead) && watari2.states.contains(State::Dead));

        // kill L again, do not sever links, and do not allow chaining
        quick_kill(&mut eng, 6, false, false, l_id);

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
        assert!(watari1.states.contains(State::Dead) && !watari2.states.contains(State::Dead));
    }

    #[test]
    fn test_ability_links() {}
}
