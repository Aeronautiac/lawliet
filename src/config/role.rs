use std::collections::BTreeMap;

use crate::config::ability::{AbilityIdentifier, AbilityName};

// TODO:
// Add passives such as:
// - Wanted (silent prosecutable off spawn)
// - Contact logs (with variants such as full, even, odd)
// - Jury duty (news anchor gets two votes)
// Passives are much more simple to implement than active abilities, but they are still
// stateful. They are not simple flags. Certain passives are transferrable (contact logs for example).
// They are also fairly versatile. A passive can act is basically a slightly more complex flag that
// does stuff like allow players to add people to the news or make them silent prosecutable off spawn.
// The ability system as a whole can likely be generalized even further. Abilities have the exact
// same ownership semantics. The only difference is that they have additional state to handle active
// usage.

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy)]
pub enum Role {
    Kira,
    SecondKira,
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

pub struct RoleAbility {
    pub identifier: AbilityIdentifier,
    pub transferrable: bool,
}

pub struct RoleConfig {
    pub abilities: Vec<RoleAbility>,
}

pub type RoleConfigMap = BTreeMap<Role, RoleConfig>;

pub fn default_role_config() -> RoleConfigMap {
    let mut map = RoleConfigMap::new();

    map.insert(
        Role::Kira,
        RoleConfig {
            abilities: vec![
                RoleAbility {
                    identifier: AbilityIdentifier {
                        name: AbilityName::UnderTheRadar,
                        variant: 0,
                    },
                    transferrable: false,
                },
                RoleAbility {
                    identifier: AbilityIdentifier {
                        name: AbilityName::AnonymousAnnouncement,
                        variant: 0,
                    },
                    transferrable: false,
                },
            ],
        },
    );

    map.insert(
        Role::SecondKira,
        RoleConfig {
            abilities: vec![
                RoleAbility {
                    identifier: AbilityIdentifier {
                        name: AbilityName::AnonymousAnnouncement,
                        variant: 0,
                    },
                    transferrable: false,
                },
                RoleAbility {
                    identifier: AbilityIdentifier {
                        name: AbilityName::UnderTheRadar,
                        variant: 0,
                    },
                    transferrable: false,
                },
                RoleAbility {
                    identifier: AbilityIdentifier {
                        name: AbilityName::KiraConnection,
                        variant: 0,
                    },
                    transferrable: false,
                },
                RoleAbility {
                    identifier: AbilityIdentifier {
                        name: AbilityName::TrueNameReveal,
                        variant: 0,
                    },
                    transferrable: false,
                },
                RoleAbility {
                    identifier: AbilityIdentifier {
                        name: AbilityName::NotebookReveal,
                        variant: 0,
                    },
                    transferrable: false,
                },
            ],
        },
    );

    map.insert(
        Role::L,
        RoleConfig {
            abilities: vec![
                RoleAbility {
                    identifier: AbilityIdentifier {
                        name: AbilityName::AnonymousAnnouncement,
                        variant: 0,
                    },
                    transferrable: false,
                },
                RoleAbility {
                    identifier: AbilityIdentifier {
                        name: AbilityName::AnonymousProsecution,
                        variant: 0,
                    },
                    transferrable: false,
                },
            ],
        },
    );

    map.insert(
        Role::Watari,
        RoleConfig {
            abilities: vec![
                RoleAbility {
                    identifier: AbilityIdentifier {
                        name: AbilityName::Bug,
                        variant: 0,
                    },
                    transferrable: true,
                },
                RoleAbility {
                    identifier: AbilityIdentifier {
                        name: AbilityName::AnonymousContact,
                        variant: 0,
                    },
                    transferrable: false,
                },
            ],
        },
    );

    map.insert(
        Role::BeyondBirthday,
        RoleConfig {
            abilities: vec![
                RoleAbility {
                    identifier: AbilityIdentifier {
                        name: AbilityName::Pseudocide,
                        variant: 0,
                    },
                    transferrable: false,
                },
                RoleAbility {
                    identifier: AbilityIdentifier {
                        name: AbilityName::TrueNameReveal,
                        variant: 0,
                    },
                    transferrable: false,
                },
                RoleAbility {
                    identifier: AbilityIdentifier {
                        name: AbilityName::NotebookReveal,
                        variant: 0,
                    },
                    transferrable: false,
                },
            ],
        },
    );

    map.insert(
        Role::PrivateInvestigator,
        RoleConfig {
            abilities: vec![
                RoleAbility {
                    identifier: AbilityIdentifier {
                        name: AbilityName::Autopsy,
                        variant: 0,
                    },
                    transferrable: false,
                },
                RoleAbility {
                    identifier: AbilityIdentifier {
                        name: AbilityName::AnonymousContact,
                        variant: 0,
                    },
                    transferrable: false,
                },
            ],
        },
    );

    map.insert(
        Role::NewsAnchor,
        RoleConfig {
            abilities: vec![RoleAbility {
                identifier: AbilityIdentifier {
                    name: AbilityName::CivilianArrest,
                    variant: 0,
                },
                transferrable: false,
            }],
        },
    );

    map
}
