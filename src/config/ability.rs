use std::collections::BTreeMap;

#[derive(Debug)]
pub enum AbilityName {
    Pseudocide,
}

pub type AbilityConfigMap = BTreeMap<AbilityName, AbilityConfig>;

#[derive(Debug)]
pub struct AbilityConfig {}

pub fn default_ability_config() -> AbilityConfigMap {}
