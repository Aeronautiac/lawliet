use indexmap::IndexMap;

use crate::{
    chargepool::{PoolLinkType, PoolSpecifier},
    common::{IterationCount, LinkWeight, Variant},
    config::{actor::ActorChargePoolName, world::WorldChargePoolName},
};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum AbilityCategory {
    Supernatural,
    Physical,
}

#[derive(Hash, Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum AbilityName {
    Contact,
    AnonymousContact,
    FalseAnonymousContact,
    AnonymousAnnouncement,
    FabricateLounge,
    Pseudocide,
    Bug,
    TapIn,
    Blackout,
    ShinigamiSacrifice,
    BackgroundCheck,
    CivilianArrest,
    UnlawfulArrest,
    UnderTheRadar,
    KiraConnection,
    AnonymousProsecution,
    Autopsy,
    Ipp,
    TrueNameReroll,
    PublicKidnap,
    AnonymousKidnap,
    TrueNameReveal,
    NotebookReveal,
    Gun,
}

#[derive(Hash, Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct AbilityIdentifier {
    pub name: AbilityName,
    pub variant: Variant,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum ConfigPoolLinkDetails {
    Individual(PoolSpecifier),  // the ability creates its own charge pool
    Actor(ActorChargePoolName), // actors and the world have a map of pool names to charge pools
    World(WorldChargePoolName),
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct ConfigPoolLink {
    pub weight: LinkWeight,
    pub link_type: PoolLinkType,
    pub details: ConfigPoolLinkDetails,
}

fn identifier(name: AbilityName, variant: Variant) -> AbilityIdentifier {
    AbilityIdentifier { name, variant }
}

pub type AbilityConfigMap = IndexMap<AbilityIdentifier, AbilityConfig>;

#[derive(Debug)]
pub struct AbilityConfig {
    pub default_links: Vec<ConfigPoolLink>, // the charge pools
    pub category: AbilityCategory,
}

// TODO:
// - Instead of trying to match an ability's pool to config to figure out what the base values are,
// just go loop through abilities and change their base value fields when config changes (take into
// consideration that a new pool may have been added by the host. Consider only the pools that match
// config entries.)

// Ability must not have multiple individual links and must not have multiple links to the same pool
pub fn default_ability_config() -> AbilityConfigMap {
    let mut map: AbilityConfigMap = IndexMap::new();

    map.insert(
        identifier(AbilityName::Contact, 0),
        AbilityConfig {
            category: AbilityCategory::Physical,
            default_links: vec![ConfigPoolLink {
                link_type: PoolLinkType::Limit,
                weight: 1,
                details: ConfigPoolLinkDetails::Actor(ActorChargePoolName::Contact),
            }],
        },
    );

    map.insert(
        identifier(AbilityName::AnonymousContact, 0),
        AbilityConfig {
            category: AbilityCategory::Physical,
            default_links: vec![ConfigPoolLink {
                link_type: PoolLinkType::Limit,
                weight: 1,
                details: ConfigPoolLinkDetails::Actor(ActorChargePoolName::Contact),
            }],
        },
    );

    map.insert(
        identifier(AbilityName::FalseAnonymousContact, 0),
        AbilityConfig {
            category: AbilityCategory::Physical,
            default_links: vec![ConfigPoolLink {
                link_type: PoolLinkType::Limit,
                weight: 1,
                details: ConfigPoolLinkDetails::Actor(ActorChargePoolName::Contact),
            }],
        },
    );

    map.insert(
        identifier(AbilityName::AnonymousAnnouncement, 0),
        AbilityConfig {
            category: AbilityCategory::Physical,
            default_links: vec![ConfigPoolLink {
                link_type: PoolLinkType::Limit,
                weight: 1,
                details: ConfigPoolLinkDetails::Individual(PoolSpecifier {
                    charges: 2,
                    reset_time: 1,
                }),
            }],
        },
    );

    map.insert(
        identifier(AbilityName::FabricateLounge, 0),
        AbilityConfig {
            category: AbilityCategory::Physical,
            default_links: vec![ConfigPoolLink {
                link_type: PoolLinkType::Limit,
                weight: 1,
                details: ConfigPoolLinkDetails::Individual(PoolSpecifier {
                    charges: 2,
                    reset_time: 1,
                }),
            }],
        },
    );

    map.insert(
        identifier(AbilityName::Pseudocide, 0),
        AbilityConfig {
            category: AbilityCategory::Physical,
            default_links: vec![ConfigPoolLink {
                link_type: PoolLinkType::Limit,
                weight: 1,
                details: ConfigPoolLinkDetails::Individual(PoolSpecifier {
                    charges: 1,
                    reset_time: 2,
                }),
            }],
        },
    );

    map.insert(
        identifier(AbilityName::Bug, 0),
        AbilityConfig {
            category: AbilityCategory::Physical,
            default_links: vec![ConfigPoolLink {
                link_type: PoolLinkType::Limit,
                weight: 1,
                details: ConfigPoolLinkDetails::Individual(PoolSpecifier {
                    charges: 1,
                    reset_time: 2,
                }),
            }],
        },
    );

    // full channel variant
    map.insert(
        identifier(AbilityName::TapIn, 0),
        AbilityConfig {
            category: AbilityCategory::Physical,
            default_links: vec![ConfigPoolLink {
                link_type: PoolLinkType::Limit,
                weight: 1,
                details: ConfigPoolLinkDetails::Individual(PoolSpecifier {
                    charges: 1,
                    reset_time: 1,
                }),
            }],
        },
    );

    // nerfed variant
    map.insert(
        identifier(AbilityName::TapIn, 1),
        AbilityConfig {
            category: AbilityCategory::Physical,
            default_links: vec![ConfigPoolLink {
                link_type: PoolLinkType::Limit,
                weight: 1,
                details: ConfigPoolLinkDetails::Individual(PoolSpecifier {
                    charges: 1,
                    reset_time: 1,
                }),
            }],
        },
    );

    map.insert(
        identifier(AbilityName::ShinigamiSacrifice, 0),
        AbilityConfig {
            category: AbilityCategory::Supernatural,
            default_links: vec![ConfigPoolLink {
                link_type: PoolLinkType::Limit,
                weight: 1,
                details: ConfigPoolLinkDetails::Individual(PoolSpecifier {
                    charges: 1,
                    reset_time: 1,
                }),
            }],
        },
    );

    map.insert(
        identifier(AbilityName::BackgroundCheck, 0),
        AbilityConfig {
            category: AbilityCategory::Supernatural,
            default_links: vec![ConfigPoolLink {
                link_type: PoolLinkType::Limit,
                weight: 1,
                details: ConfigPoolLinkDetails::Individual(PoolSpecifier {
                    charges: 1,
                    reset_time: 1,
                }),
            }],
        },
    );

    map.insert(
        identifier(AbilityName::CivilianArrest, 0),
        AbilityConfig {
            category: AbilityCategory::Physical,
            default_links: vec![ConfigPoolLink {
                link_type: PoolLinkType::Limit,
                weight: 1,
                details: ConfigPoolLinkDetails::Individual(PoolSpecifier {
                    charges: 1,
                    reset_time: 1,
                }),
            }],
        },
    );

    map.insert(
        identifier(AbilityName::UnlawfulArrest, 0),
        AbilityConfig {
            category: AbilityCategory::Physical,
            default_links: vec![ConfigPoolLink {
                link_type: PoolLinkType::Limit,
                weight: 1,
                details: ConfigPoolLinkDetails::Individual(PoolSpecifier {
                    charges: 1,
                    reset_time: 1,
                }),
            }],
        },
    );

    map.insert(
        identifier(AbilityName::UnderTheRadar, 0),
        AbilityConfig {
            category: AbilityCategory::Supernatural,
            default_links: vec![ConfigPoolLink {
                link_type: PoolLinkType::Limit,
                weight: 1,
                details: ConfigPoolLinkDetails::Individual(PoolSpecifier {
                    charges: 1,
                    reset_time: IterationCount::MAX,
                }),
            }],
        },
    );

    map.insert(
        identifier(AbilityName::KiraConnection, 0),
        AbilityConfig {
            category: AbilityCategory::Supernatural,
            default_links: vec![ConfigPoolLink {
                link_type: PoolLinkType::Limit,
                weight: 1,
                details: ConfigPoolLinkDetails::Individual(PoolSpecifier {
                    charges: 1,
                    reset_time: 1,
                }),
            }],
        },
    );

    map.insert(
        identifier(AbilityName::AnonymousProsecution, 0),
        AbilityConfig {
            category: AbilityCategory::Physical,
            default_links: vec![ConfigPoolLink {
                link_type: PoolLinkType::Limit,
                weight: 1,
                details: ConfigPoolLinkDetails::Individual(PoolSpecifier {
                    charges: 1,
                    reset_time: IterationCount::MAX,
                }),
            }],
        },
    );

    map.insert(
        identifier(AbilityName::Autopsy, 0),
        AbilityConfig {
            category: AbilityCategory::Physical,
            default_links: vec![ConfigPoolLink {
                link_type: PoolLinkType::Limit,
                weight: 1,
                details: ConfigPoolLinkDetails::Individual(PoolSpecifier {
                    charges: 1,
                    reset_time: 1,
                }),
            }],
        },
    );

    map.insert(
        identifier(AbilityName::Ipp, 0),
        AbilityConfig {
            category: AbilityCategory::Physical,
            default_links: vec![ConfigPoolLink {
                link_type: PoolLinkType::Limit,
                weight: 1,
                details: ConfigPoolLinkDetails::Individual(PoolSpecifier {
                    charges: 1,
                    reset_time: 1,
                }),
            }],
        },
    );

    map.insert(
        identifier(AbilityName::TrueNameReroll, 0),
        AbilityConfig {
            category: AbilityCategory::Physical,
            default_links: vec![ConfigPoolLink {
                link_type: PoolLinkType::Limit,
                weight: 1,
                details: ConfigPoolLinkDetails::Individual(PoolSpecifier {
                    charges: 1,
                    reset_time: IterationCount::MAX,
                }),
            }],
        },
    );

    map.insert(
        identifier(AbilityName::PublicKidnap, 0),
        AbilityConfig {
            category: AbilityCategory::Physical,
            default_links: vec![ConfigPoolLink {
                link_type: PoolLinkType::Limit,
                weight: 1,
                details: ConfigPoolLinkDetails::Individual(PoolSpecifier {
                    charges: 1,
                    reset_time: 1,
                }),
            }],
        },
    );

    map.insert(
        identifier(AbilityName::AnonymousKidnap, 0),
        AbilityConfig {
            category: AbilityCategory::Physical,
            default_links: vec![ConfigPoolLink {
                link_type: PoolLinkType::Limit,
                weight: 1,
                details: ConfigPoolLinkDetails::Individual(PoolSpecifier {
                    charges: 1,
                    reset_time: 1,
                }),
            }],
        },
    );

    map.insert(
        identifier(AbilityName::Blackout, 0),
        AbilityConfig {
            category: AbilityCategory::Physical,
            default_links: vec![ConfigPoolLink {
                link_type: PoolLinkType::Limit,
                weight: 1,
                details: ConfigPoolLinkDetails::Individual(PoolSpecifier {
                    charges: 1,
                    reset_time: IterationCount::MAX,
                }),
            }],
        },
    );

    map.insert(
        identifier(AbilityName::TrueNameReveal, 0),
        AbilityConfig {
            category: AbilityCategory::Supernatural,
            default_links: vec![ConfigPoolLink {
                link_type: PoolLinkType::Limit,
                weight: 1,
                details: ConfigPoolLinkDetails::Individual(PoolSpecifier {
                    charges: 2,
                    reset_time: 1,
                }),
            }],
        },
    );

    map.insert(
        identifier(AbilityName::NotebookReveal, 0),
        AbilityConfig {
            category: AbilityCategory::Supernatural,
            default_links: vec![ConfigPoolLink {
                link_type: PoolLinkType::Limit,
                weight: 1,
                details: ConfigPoolLinkDetails::Individual(PoolSpecifier {
                    charges: 2,
                    reset_time: 1,
                }),
            }],
        },
    );

    map
}
