use std::collections::BTreeMap;

use crate::{
    actor::ActorLinkType,
    config::ability::{AbilityIdentifier, AbilityName},
    passive::{ContactLogType, PassiveType},
};

// TODO:
// Add actor link configurations. Actor links are unidirectional and allow things like sharing passives
// and linking deaths.
// For example, L and Watari have two links. L has a life link to Watari, and Watari has a passive
// link to L.
// The passive link enables their shared contact log ability. If Watari dies (and link
// acknowledgement is set to true in the kill action), the links are severed and L loses the ability to view contact logs.
// L's life link ensures that Watari dies if L dies.
// If L is revived, Watari is also revived.
// In the case that there are multiple Wataris, all Wataris will die when L dies, and all are
// revived when L is revived.
// In the case that a Watari dies, L will still have contact logs.
// Link acknowledgement is an option in the kill action which determines if link behaviour is
// considered or not.

// TODO:
// Certain roles spawn with death notes (either real or fake).
// Death notes shall be considered volatile until their true owner is changed.

// TODO:
// Add organization configurations. Certain roles spawn in organizations with a certain rank.
// For example, Near spawns as the leader of the SPK.

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

pub struct RolePassive {
    pub passive_type: PassiveType,
    pub transferrable: bool,
}

pub struct RoleAbility {
    pub identifier: AbilityIdentifier,
    pub transferrable: bool,
}

pub struct RoleNotebook {
    pub fake: bool,
}

pub struct RoleLink {
    pub role: Role,
    pub link_type: ActorLinkType,
}

pub struct RoleConfig {
    pub abilities: Vec<RoleAbility>,
    pub passives: Vec<RolePassive>,
    pub notebooks: Vec<RoleNotebook>,
    pub actor_links: Vec<RoleLink>,
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
            passives: vec![],
            notebooks: vec![RoleNotebook { fake: false }],
            actor_links: vec![],
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
            passives: vec![RolePassive {
                passive_type: PassiveType::OwnedNotebookBlock,
                transferrable: false,
            }],
            notebooks: vec![RoleNotebook { fake: false }],
            actor_links: vec![],
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
            passives: vec![],
            notebooks: vec![],
            actor_links: vec![
                RoleLink {
                    role: Role::Watari,
                    link_type: ActorLinkType::Life,
                },
                RoleLink {
                    role: Role::Watari,
                    link_type: ActorLinkType::Passive,
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
            passives: vec![RolePassive {
                passive_type: PassiveType::ContactLogs(ContactLogType::Full),
                transferrable: true,
            }],
            notebooks: vec![],
            actor_links: vec![],
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
            passives: vec![],
            notebooks: vec![],
            actor_links: vec![],
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
            passives: vec![],
            notebooks: vec![],
            actor_links: vec![],
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
            passives: vec![RolePassive {
                passive_type: PassiveType::JuryDuty,
                transferrable: false,
            }],
            notebooks: vec![],
            actor_links: vec![],
        },
    );

    map.insert(
        Role::Civilian,
        RoleConfig {
            abilities: vec![],
            passives: vec![],
            notebooks: vec![],
            actor_links: vec![],
        },
    );

    map.insert(
        Role::RogueCivilian,
        RoleConfig {
            abilities: vec![],
            passives: vec![],
            notebooks: vec![RoleNotebook { fake: false }],
            actor_links: vec![],
        },
    );

    map.insert(
        Role::Poser,
        RoleConfig {
            abilities: vec![
                RoleAbility {
                    identifier: AbilityIdentifier {
                        name: AbilityName::FalseAnonymousContact,
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
            passives: vec![],
            notebooks: vec![],
            actor_links: vec![],
        },
    );

    map.insert(
        Role::ConArtist,
        RoleConfig {
            abilities: vec![RoleAbility {
                identifier: AbilityIdentifier {
                    name: AbilityName::FabricateLounge,
                    variant: 0,
                },
                transferrable: false,
            }],
            passives: vec![],
            notebooks: vec![RoleNotebook { fake: true }],
            actor_links: vec![],
        },
    );

    map.insert(
        Role::WantedCivilian,
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
                        name: AbilityName::TapIn,
                        variant: 1,
                    },
                    transferrable: true,
                },
            ],
            passives: vec![RolePassive {
                passive_type: PassiveType::Wanted,
                transferrable: false,
            }],
            notebooks: vec![],
            actor_links: vec![],
        },
    );

    map.insert(
        Role::Near,
        RoleConfig {
            abilities: vec![RoleAbility {
                identifier: AbilityIdentifier {
                    name: AbilityName::AnonymousAnnouncement,
                    variant: 0,
                },
                transferrable: false,
            }],
            passives: vec![RolePassive {
                passive_type: PassiveType::ContactLogs(ContactLogType::Even),
                transferrable: true,
            }],
            notebooks: vec![RoleNotebook { fake: true }],
            actor_links: vec![],
        },
    );

    map.insert(
        Role::Mello,
        RoleConfig {
            abilities: vec![RoleAbility {
                identifier: AbilityIdentifier {
                    name: AbilityName::AnonymousAnnouncement,
                    variant: 0,
                },
                transferrable: false,
            }],
            passives: vec![RolePassive {
                passive_type: PassiveType::ContactLogs(ContactLogType::Odd),
                transferrable: true,
            }],
            notebooks: vec![],
            actor_links: vec![],
        },
    );

    map
}
