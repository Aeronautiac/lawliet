use crate::{Timestamp, config::ability::AbilityIdentifier};

pub struct DefaultConfig {
    pub death_message: String,
    pub pseudocide_duration: Timestamp, // milliseconds
    pub universal_abilities: Vec<AbilityIdentifier>, // the abilities that everyone gets regardless
                                        // of role
}

pub fn default_defaults() -> DefaultConfig {
    DefaultConfig {
        death_message: "They died from a sudden heart attack.".into(),
        pseudocide_duration: 24 * 60 * 60 * 1000, // 24 hrs
        universal_abilities: vec![AbilityIdentifier {
            name: crate::config::ability::AbilityName::Contact,
            variant: 0,
        }],
    }
}
