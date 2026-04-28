/*
* lawliet
* deterministic headless engine for a death note social deduction game
* - process atomic actions
* - simulate a timeline
* - maintain an internal priority queue of future events for job scheduling
* - most actions are executed directly
* - actions may invoke other actions
* - adding an event to the priority queue requires an action
* - handle permissions
* - handle game init and configuration through a series of actions
* - handle pure game logic
* - to modify an existing action, simply add a new action handler and call it from the yagami hypervisor rather
*   than the old variant
*/

// config is dynamic and determines many aspects of game behaviour during action processing
// changing config is an action
// lawliet begins with default config, but you may change aspects of config with different actions
//
// a player is a distinct stateful actor
//
// an organization is similar to a player
// it is an instance of an actor
// it is a group of players, and its behaviour is identified by its type similarly to how players
// have roles
//
// a role is an identifier
// a role is associated with some set of abilities in the config struct
// the presence of a role within an organization can influence what that organization can do
// a role can override default ability values in config
// when abilities are granted, it will prioritize the role's values over the ability's values
//
// abilities are distinct stateful objects
// abilities have rules and permissions
// abilities can be owned by different kinds of actors, can be transferred between actors, and can
// be owned by multiple actors
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
// layer.
// game state is not snapshotted. instead, it is reconstructed from a saved actions sequence.
// yagami stores actions into a buffer. when certain conditions are met (timer, significant action,
// full buffer, etc...) it flushes it to a postgres db.

mod ability;
mod action;
mod actor;
mod command;
mod common;
mod config;
mod engine;
mod notebook;
mod world;

pub use common::{ID, Timestamp};

#[cfg(test)]
mod tests {
    use crate::{
        ID, Timestamp,
        action::{
            Action, ActionActor, ActionRequest, ActionResult,
            add_player::{AddPlayer, AddPlayerResponse},
            kill::Kill,
        },
        actor::{ActorType, state::State},
        config::role::Role,
        engine::Engine,
    };

    // fn add_player(
    //     eng: &mut Engine,
    //     timestamp: Timestamp,
    //     true_name: &str,
    //     starting_role: Role,
    // ) -> ActionResult {
    //     eng.execute(ActionRequest {
    //         timestamp,
    //         actor: ActionActor::System,
    //         payload: Action::AddPlayer(AddPlayer {
    //             true_name: String::from(true_name),
    //             starting_role,
    //         }),
    //     })
    // }
    //
    // fn player_response_data(data: ResponseData) -> AddPlayerResponse {
    //     let ResponseData::AddPlayer(player_data) = data else {
    //         unreachable!()
    //     };
    //     player_data
    // }
    //
    // fn kill(
    //     eng: &mut Engine,
    //     timestamp: Timestamp,
    //     player_id: ID,
    //     killer_id: Option<ID>,
    //     death_message: Option<String>,
    // ) -> ActionResult {
    //     eng.execute(ActionRequest {
    //         timestamp,
    //         actor: ActionActor::System,
    //         payload: Action::Kill(Kill {
    //             target_id: player_id,
    //             death_message,
    //             killer_id,
    //             silent: false,
    //         }),
    //     })
    // }

    // #[test]
    // fn test_add_single_player() {
    //     let mut eng = Engine::new();
    //
    //     let john_result = add_player(&mut eng, 0, "John Pork", Role::NewsAnchor).unwrap();
    //     let response_data = player_response_data(john_result.data);
    //     assert!(eng.world.actors.contains_key(&response_data.id));
    //
    //     let ActorType::Player(player) =
    //         &eng.world.actors.get(&response_data.id).unwrap().actor_type
    //     else {
    //         unreachable!();
    //     };
    //     assert!(&*player.true_name == "john pork");
    //     assert!(player.role == Role::NewsAnchor);
    // }
    //
    // #[test]
    // fn add_duplicate_player() {
    //     let mut eng = Engine::new();
    //
    //     let john_result = add_player(&mut eng, 0, "John Pork", Role::NewsAnchor).unwrap();
    //     let response_data = player_response_data(john_result.data);
    //
    //     // adding another player with the same true name should error
    //     let second_result = add_player(&mut eng, 0, "John Pork", Role::Poser);
    //     assert!(second_result.is_err());
    //
    //     // ensure that the data didn't change
    //     let ActorType::Player(player) =
    //         &eng.world.actors.get(&response_data.id).unwrap().actor_type
    //     else {
    //         unreachable!();
    //     };
    //     assert!(&*player.true_name == "john pork");
    //     assert!(player.role == Role::NewsAnchor);
    // }
    //
    // // later test command output
    // #[test]
    // fn kill_player() {
    //     let mut eng = Engine::new();
    //
    //     let john_result = add_player(&mut eng, 0, "John Pork", Role::NewsAnchor).unwrap();
    //     let response_data = player_response_data(john_result.data);
    //     assert!(
    //         !eng.world
    //             .actors
    //             .get(&response_data.id)
    //             .unwrap()
    //             .states
    //             .contains(State::Dead)
    //     );
    //
    //     let _ = kill(
    //         &mut eng,
    //         0,
    //         response_data.id,
    //         None,
    //         Some("Heart attack...".to_string()),
    //     )
    //     .unwrap();
    //     assert!(
    //         eng.world
    //             .actors
    //             .get(&response_data.id)
    //             .unwrap()
    //             .states
    //             .contains(State::Dead)
    //     );
    // }
}
