use std::collections::BTreeMap;

use crate::{ability::AbilityLinkType, common::Variant};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
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
    IPP,
    TrueNameReroll,
    PublicKidnap,
    AnonymousKidnap,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct AbilityIdentifier {
    name: AbilityName,
    variant: Variant,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct ConfigAbilityLink {
    link_type: AbilityLinkType,
    identifier: AbilityIdentifier,
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
// ability objects and as such will be specified in role config. It is denoted with a boolean flag.
//
// If an ability with an existing link is transferred, all existing links are broken, and new links
// are created after the transfer.

// TODO: Add ability categories like "Supernatural", "Physical", etc... An ability's source of truth
// for category is the config struct.
#[derive(Debug)]
pub struct AbilityConfig {
    reset_time: u16,
    base_charges: u16,
    default_links: Vec<ConfigAbilityLink>, // what this should ability link to if given the
                                           // opportunity
}

pub fn default_ability_config() -> AbilityConfigMap {
    let mut map: AbilityConfigMap = BTreeMap::new();

    map.insert(
        identifier(AbilityName::Contact, 0),
        AbilityConfig {
            base_charges: 5,
            reset_time: 1,
            default_links: vec![],
        },
    );

    map.insert(
        identifier(AbilityName::AnonymousContact, 0),
        AbilityConfig {
            base_charges: 1,
            reset_time: 1,
            default_links: vec![ConfigAbilityLink {
                identifier: identifier(AbilityName::Contact, 0),
                link_type: AbilityLinkType::Limit,
            }],
        },
    );

    map.insert(
        identifier(AbilityName::FalseAnonymousContact, 0),
        AbilityConfig {
            base_charges: 1,
            reset_time: 1,
            default_links: vec![ConfigAbilityLink {
                identifier: identifier(AbilityName::Contact, 0),
                link_type: AbilityLinkType::Limit,
            }],
        },
    );

    map.insert(
        identifier(AbilityName::AnonymousAnnouncement, 0),
        AbilityConfig {
            base_charges: 2,
            reset_time: 1,
            default_links: vec![],
        },
    );

    map.insert(
        identifier(AbilityName::FabricateLounge, 0),
        AbilityConfig {
            base_charges: 2,
            reset_time: 1,
            default_links: vec![],
        },
    );

    map.insert(
        identifier(AbilityName::Pseudocide, 0),
        AbilityConfig {
            base_charges: 1,
            reset_time: 2,
            default_links: vec![],
        },
    );

    map
}
