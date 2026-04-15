/*
* lawliet
* deterministic headless engine
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
// there is a map of player ids to player structs
// player permissions are determined by a combination of their role and restrictions
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
// abilities can be owned by different kinds of actors, and can be transferred between actors
// abilities have an ability type which is simply an identifier
// abilities may have a variant to further narrow behaviour if necessary
// abilities have a "category".
//
// a state is a simple identity with a set of restrictions associated with it
// restrictions can be added without states, but adding a state will add the restrictions
// associated with that state in config
//
// a restriction is an attempt to block some specific permission or set of permissions
// actors have source maps of "sources" to restrictions. they utilize bitmaps for utility and minor
// performance gains.
// for example, "incarcerated": [noabilitycategory(abilitycategory), nocontact] restriction is an example of a restriction.
//
// yagami hosts multiple lawliet instances and communicates via ipc. it also acts as a persistence
// layer.
// game state is not snapshotted. instead, it is reconstructed from a saved actions sequence.
// yagami stores actions into a buffer. when certain conditions are met (timer, significant action,
// full buffer, etc...) it flushes it to a postgres db.

mod ability;
mod action;
mod actor;
mod common;
mod config;
mod engine;
mod notebook;
mod world;

pub use common::{ID, Timestamp};

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
}
