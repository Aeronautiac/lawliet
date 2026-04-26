pub mod ability;
pub mod organization;
pub mod role;
pub mod ruleset;
pub mod state;

use ability::AbilityConfig;

use crate::config::{
    role::{RoleConfigMap, default_role_config},
    state::{StateRestrictionMap, default_state_restrictions},
};

// these should be maps
pub struct Config {
    pub roles: RoleConfigMap,
    pub abilities: AbilityConfig,
    pub state_restrictions: StateRestrictionMap,
}

impl Config {
    pub fn new() -> Self {
        Config {
            roles: default_role_config(),
            abilities: vec![],
            state_restrictions: default_state_restrictions(),
        }
    }
}
