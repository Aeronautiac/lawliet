pub mod ability;
pub mod organization;
pub mod role;

use ability::AbilityConfig;
use role::RoleConfig;

// these should be maps
pub struct Config {
    pub roles: Vec<RoleConfig>,
    pub abilities: Vec<AbilityConfig>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            roles: vec![],
            abilities: vec![],
        }
    }
}
