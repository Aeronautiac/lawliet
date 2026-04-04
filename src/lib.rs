/*
* LAWLIET
* Deterministic headless engine
* - Process atomic actions
* - Simulate a timeline
* - Maintain an internal priority queue of future events for job scheduling
* - Most actions are executed directly
* - Actions may invoke other actions
* - Adding an event to the priority queue requires an action
* - Handle permissions
* - Handle game init and configuration through a series of actions
* - Handle pure game logic
* - To modify an existing action, simply add a new action handler and call it from the YAGAMI hypervisor rather
*   than the old variant
*/

// Config is dynamic and determines many aspects of game behaviour during action processing
// changing config is an action
// LAWLIET begins with default config, but you may change aspects of config with different actions
//
// A player is a distinct stateful actor
// There is a map of player ids to player structs
// Player permissions are determined by a combination of their role and restrictions
//
// An organization is similar to a player
// it is an instance of an actor
// it is a group of players, and its behaviour is identified by its type similarly to how players
// have roles
//
// A role is an identifier
// A role is associated with some set of abilities in the config struct
// The presence of a role within an organization can influence what that organization can do
// A role can override default ability values in config
// When abilities are granted, it will prioritize the role's values over the ability's values
//
// Abilities are distinct stateful objects
// Abilities have rules and permissions
// Abilities can be owned by different kinds of actors, and can be transferred between actors
// Abilities have an ability type which is simply an identifier
// Abilities may have a variant to further narrow behaviour if necessary
// Abilities have a "category".
//
// A state is a simple identity with a set of restrictions associated with it
// restrictions can be added without states, but adding a state will add the restrictions
// associated with that state in config
//
// A restriction is an attempt to block some specific permission or set of permissions
// Actors have source maps of "sources" to restrictions. They utilize bitmaps for utility and minor
// performance gains.
// For example, "Incarcerated": [NoAbilityCategory(AbilityCategory), NoContact] restriction is an example of a restriction.
//
// A state is just a set of restrictions with an identifier
// There are a set of pre-defined states in the default config, but you can define your own as well

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
