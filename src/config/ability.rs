use std::collections::BTreeMap;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum AbilityName {
    Pseudocide,
}

pub type AbilityConfigMap = BTreeMap<AbilityName, AbilityConfig>;

#[derive(Debug)]
pub struct AbilityConfig {}

pub fn default_ability_config() -> AbilityConfigMap {
    let map: AbilityConfigMap = BTreeMap::new();
    map
}
