use std::collections::BTreeMap;

use crate::{
    ability::AbilityLinkType,
    common::{IterationCount, LinkWeight, Variant},
};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum AbilityCategory {
    Supernatural,
    Physical,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum AbilityName {
    Contact,
    Pseudocide,
    AnonymousContact,
    FalseAnonymousContact,
    AnonymousAnnouncement,
    FabricateLounge,
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
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct AbilityIdentifier {
    pub name: AbilityName,
    pub variant: Variant,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct ConfigAbilityLink {
    pub link_type: AbilityLinkType,
    pub identifier: AbilityIdentifier,
    pub weight: LinkWeight,
}

fn identifier(name: AbilityName, variant: Variant) -> AbilityIdentifier {
    AbilityIdentifier { name, variant }
}

pub type AbilityConfigMap = BTreeMap<AbilityIdentifier, AbilityConfig>;

// Ability objects will themselves have an "require roles" vector in the case of an organization
// ability which is locked behind the presence of certain roles within an org. Ability config will
// have nothing to do with this. Only org config will.
//
// Certain abilities may be transferred between players on kill. This is specific to individual
// ability objects and as such will be specified in role config to be applied on creation. It is denoted with a boolean flag.
//
// If an ability with an existing link is transferred, all existing links are broken, and new links
// are created after the transfer.
//
// Must add link weight to ability config to allow for things like group chat creation using up all
// contact charges.

#[derive(Debug)]
pub struct AbilityConfig {
    pub reset_time: IterationCount,
    pub base_charges: u16,
    pub default_links: Vec<ConfigAbilityLink>, // what this should ability link to if given the
    // opportunity
    pub category: AbilityCategory,
}

pub fn default_ability_config() -> AbilityConfigMap {
    let mut map: AbilityConfigMap = BTreeMap::new();

    map.insert(
        identifier(AbilityName::Contact, 0),
        AbilityConfig {
            category: AbilityCategory::Physical,
            base_charges: 5,
            reset_time: 1,
            default_links: vec![],
        },
    );

    map.insert(
        identifier(AbilityName::AnonymousContact, 0),
        AbilityConfig {
            category: AbilityCategory::Physical,
            base_charges: 1,
            reset_time: 1,
            default_links: vec![ConfigAbilityLink {
                identifier: identifier(AbilityName::Contact, 0),
                link_type: AbilityLinkType::Limit,
                weight: 1,
            }],
        },
    );

    map.insert(
        identifier(AbilityName::FalseAnonymousContact, 0),
        AbilityConfig {
            category: AbilityCategory::Physical,
            base_charges: 1,
            reset_time: 1,
            default_links: vec![ConfigAbilityLink {
                identifier: identifier(AbilityName::Contact, 0),
                link_type: AbilityLinkType::Limit,
                weight: 1,
            }],
        },
    );

    map.insert(
        identifier(AbilityName::AnonymousAnnouncement, 0),
        AbilityConfig {
            category: AbilityCategory::Physical,
            base_charges: 2,
            reset_time: 1,
            default_links: vec![],
        },
    );

    map.insert(
        identifier(AbilityName::FabricateLounge, 0),
        AbilityConfig {
            category: AbilityCategory::Physical,
            base_charges: 2,
            reset_time: 1,
            default_links: vec![],
        },
    );

    map.insert(
        identifier(AbilityName::Pseudocide, 0),
        AbilityConfig {
            category: AbilityCategory::Physical,
            base_charges: 1,
            reset_time: 2,
            default_links: vec![],
        },
    );

    map.insert(
        identifier(AbilityName::Bug, 0),
        AbilityConfig {
            category: AbilityCategory::Physical,
            base_charges: 1,
            reset_time: 2,
            default_links: vec![],
        },
    );

    // full channel variant
    map.insert(
        identifier(AbilityName::TapIn, 0),
        AbilityConfig {
            category: AbilityCategory::Physical,
            base_charges: 1,
            reset_time: 1,
            default_links: vec![],
        },
    );

    // 12 hr variant (wanted civ)
    map.insert(
        identifier(AbilityName::TapIn, 1),
        AbilityConfig {
            category: AbilityCategory::Physical,
            base_charges: 1,
            reset_time: 1,
            default_links: vec![],
        },
    );

    map.insert(
        identifier(AbilityName::Blackout, 0),
        AbilityConfig {
            category: AbilityCategory::Physical,
            base_charges: 1,
            reset_time: IterationCount::MAX,
            default_links: vec![],
        },
    );

    map.insert(
        identifier(AbilityName::TrueNameReveal, 0),
        AbilityConfig {
            category: AbilityCategory::Supernatural,
            base_charges: 2,
            reset_time: 1,
            default_links: vec![ConfigAbilityLink {
                link_type: AbilityLinkType::Limit,
                identifier: AbilityIdentifier {
                    name: AbilityName::NotebookReveal,
                    variant: 0,
                },
                weight: 1,
            }],
        },
    );

    map.insert(
        identifier(AbilityName::NotebookReveal, 0),
        AbilityConfig {
            category: AbilityCategory::Supernatural,
            base_charges: 1,
            reset_time: 1,
            default_links: vec![ConfigAbilityLink {
                link_type: AbilityLinkType::Limit,
                identifier: AbilityIdentifier {
                    name: AbilityName::TrueNameReveal,
                    variant: 0,
                },
                weight: 2,
            }],
        },
    );

    map
}
