pub mod ability;
pub mod organization;
pub mod role;
pub mod state;

use ability::AbilityConfig;
use role::RoleConfig;

use crate::config::state::{StateRestrictionMap, default_state_restrictions};

// these should be maps
pub struct Config {
    pub roles: Vec<RoleConfig>,
    pub abilities: Vec<AbilityConfig>,
    pub state_restrictions: StateRestrictionMap,
}

impl Config {
    pub fn new() -> Self {
        Config {
            roles: vec![],
            abilities: vec![],
            state_restrictions: default_state_restrictions(),
        }
    }
}
