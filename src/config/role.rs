use std::collections::{BTreeMap, BTreeSet};

use crate::config::ability::AbilityName;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Role {
    Kira,
    Kira2nd,
    L,
    Watari,
    BeyondBirthday,
    PrivateInvestigator,
    NewsAnchor,
    Civilian,
    RogueCivilian,
    Poser,
    ConArtist,
    WantedCivilian,
    Near,
    Mello,
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct AbilityIdentifier {
    pub ability_name: AbilityName,
    pub variant: u8,
}

pub struct RoleConfig {
    pub charges: u8,
    pub cooldown: u8,
    pub abilities: BTreeSet<AbilityName>,
}

pub type RoleConfigMap = BTreeMap<Role, RoleConfig>;

pub fn default_role_config() -> RoleConfigMap {
    let map = RoleConfigMap::new();
    map
}
